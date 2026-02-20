//! Gateway Lite implementation.
//!
//! Implements `circuits/marketplace/industrial-gateway-lite.escir.yaml`
//!
//! Features:
//! - MODBUS TCP only
//! - Up to 10 devices
//! - Up to 256 registers
//! - Up to 64 alarms
//! - Full StreamSight telemetry

use crate::config::{GatewayConfig, DeviceConfig, RegisterConfig};
use crate::emitter::{EmitterConfig, StreamEmitter, StreamEvent, AlarmEventOutput};
use crate::protocol::{ModbusTcpClient, ModbusEvent, ModbusReadRequest};
use crate::scheduler::{PollScheduler, SchedulerConfig, PollItem, PollTrigger, PollComplete};
use crate::streamsight::{StreamSightBridge, BridgeConfig, LexEvent, TelemetryEvent, GatewayHealthEvent};
use crate::types::*;
use crate::{IndustrialError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};

/// Gateway Lite limits.
pub const MAX_DEVICES: usize = 10;
pub const MAX_REGISTERS: usize = 256;
pub const MAX_ALARMS: usize = 64;

/// Gateway Lite - Free tier industrial gateway.
pub struct GatewayLite {
    /// Configuration
    config: GatewayConfig,
    /// MODBUS clients by device_id
    clients: RwLock<HashMap<String, Arc<ModbusTcpClient>>>,
    /// Poll scheduler
    scheduler: Arc<PollScheduler>,
    /// Stream emitter
    emitter: Arc<StreamEmitter>,
    /// StreamSight bridge
    bridge: Arc<StreamSightBridge>,
    /// Start time
    start_time: RwLock<Option<Instant>>,
    /// Running flag
    running: std::sync::atomic::AtomicBool,
    /// Metrics
    metrics: RwLock<GatewayMetrics>,
    /// Event subscription channels
    event_subscribers: RwLock<Vec<mpsc::Sender<StreamEvent>>>,
    /// Alarm subscription channels
    alarm_subscribers: RwLock<Vec<mpsc::Sender<AlarmEventOutput>>>,
    /// LEX subscription channels
    lex_subscribers: RwLock<Vec<mpsc::Sender<LexEvent>>>,
}

impl GatewayLite {
    /// Creates a new Gateway Lite instance.
    pub async fn new(config: GatewayConfig) -> Result<Self> {
        // Validate limits
        if config.devices.len() > MAX_DEVICES {
            return Err(IndustrialError::LimitExceeded {
                limit_name: "devices".into(),
                max: MAX_DEVICES as u32,
                requested: config.devices.len() as u32,
            });
        }
        
        if config.registers.len() > MAX_REGISTERS {
            return Err(IndustrialError::LimitExceeded {
                limit_name: "registers".into(),
                max: MAX_REGISTERS as u32,
                requested: config.registers.len() as u32,
            });
        }
        
        if config.alarms.len() > MAX_ALARMS {
            return Err(IndustrialError::LimitExceeded {
                limit_name: "alarms".into(),
                max: MAX_ALARMS as u32,
                requested: config.alarms.len() as u32,
            });
        }
        
        // Create internal channels
        let (poll_trigger_tx, poll_trigger_rx) = mpsc::channel(256);
        let (stream_event_tx, stream_event_rx) = mpsc::channel(256);
        let (alarm_event_tx, alarm_event_rx) = mpsc::channel(64);
        let (lex_event_tx, lex_event_rx) = mpsc::channel(256);
        let (modbus_event_tx, _modbus_event_rx) = mpsc::channel::<ModbusEvent>(256);
        
        // Create scheduler
        let scheduler_config = SchedulerConfig {
            max_polls_per_second: config.settings.max_polls_per_second,
            adaptive_enabled: config.settings.adaptive_scheduling,
            backoff_factor: config.settings.backoff_factor,
            max_backoff_interval_ms: config.settings.max_backoff_interval_ms,
        };
        let scheduler = Arc::new(PollScheduler::with_trigger_channel(
            scheduler_config,
            poll_trigger_tx,
        ));
        
        // Create emitter
        let emitter_config = EmitterConfig {
            gateway_id: config.gateway_id,
            emit_on_change_only: false,
            batch_enabled: true,
            ..Default::default()
        };
        let emitter = Arc::new(StreamEmitter::new(
            emitter_config,
            stream_event_tx,
            alarm_event_tx,
        ));
        
        // Create StreamSight bridge
        let bridge_config = BridgeConfig {
            gateway_id: config.gateway_id,
            severity_filter: config.streamsight.severity_filter,
            batch_size: config.streamsight.batch_size,
            flush_interval_ms: config.streamsight.batch_interval_ms,
            sampling_rate: config.streamsight.sampling_rate,
            ..Default::default()
        };
        let bridge = Arc::new(StreamSightBridge::new(bridge_config, lex_event_tx));
        
        // Create MODBUS clients
        let mut clients = HashMap::new();
        for device in &config.devices {
            if device.enabled {
                let client = Arc::new(ModbusTcpClient::new(device.clone()));
                clients.insert(device.device_id.clone(), client);
            }
        }
        
        // Add registers to scheduler and emitter
        for register in &config.registers {
            let poll_item = PollItem::from(register);
            scheduler.add_poll(poll_item).await;
            emitter.add_register(register.clone()).await;
        }
        
        // Add alarms to emitter
        for alarm in &config.alarms {
            emitter.add_alarm(alarm.clone()).await;
        }
        
        Ok(Self {
            config,
            clients: RwLock::new(clients),
            scheduler,
            emitter,
            bridge,
            start_time: RwLock::new(None),
            running: std::sync::atomic::AtomicBool::new(false),
            metrics: RwLock::new(GatewayMetrics::default()),
            event_subscribers: RwLock::new(Vec::new()),
            alarm_subscribers: RwLock::new(Vec::new()),
            lex_subscribers: RwLock::new(Vec::new()),
        })
    }
    
    /// Returns the gateway ID.
    pub fn gateway_id(&self) -> &[u8; 32] {
        &self.config.gateway_id
    }
    
    /// Returns the gateway name.
    pub fn name(&self) -> &str {
        &self.config.name
    }
    
    /// Returns whether the gateway is running.
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Starts the gateway.
    pub async fn start(&self) -> Result<()> {
        if self.is_running() {
            return Err(IndustrialError::GatewayAlreadyRunning);
        }
        
        info!("Starting Gateway Lite: {}", self.config.name);
        
        // Connect all clients
        let clients = self.clients.read().await;
        for (device_id, client) in clients.iter() {
            match client.connect().await {
                Ok(()) => {
                    info!("Connected to device: {}", device_id);
                    self.metrics.write().await.devices_online += 1;
                }
                Err(e) => {
                    warn!("Failed to connect to {}: {}", device_id, e);
                }
            }
        }
        
        self.metrics.write().await.devices_configured = clients.len() as u16;
        drop(clients);
        
        // Mark as running
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        *self.start_time.write().await = Some(Instant::now());
        
        info!("Gateway Lite started");
        Ok(())
    }
    
    /// Stops the gateway.
    pub async fn stop(&self) -> Result<()> {
        if !self.is_running() {
            return Err(IndustrialError::GatewayNotRunning);
        }
        
        info!("Stopping Gateway Lite: {}", self.config.name);
        
        // Stop scheduler
        self.scheduler.stop();
        
        // Disconnect all clients
        let clients = self.clients.read().await;
        for (device_id, client) in clients.iter() {
            if let Err(e) = client.disconnect().await {
                warn!("Error disconnecting from {}: {}", device_id, e);
            }
        }
        
        // Flush StreamSight
        self.bridge.flush().await;
        
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        
        info!("Gateway Lite stopped");
        Ok(())
    }
    
    /// Subscribes to stream events.
    pub async fn subscribe_events(&self) -> mpsc::Receiver<StreamEvent> {
        let (tx, rx) = mpsc::channel(256);
        self.event_subscribers.write().await.push(tx);
        rx
    }
    
    /// Subscribes to alarm events.
    pub async fn subscribe_alarms(&self) -> mpsc::Receiver<AlarmEventOutput> {
        let (tx, rx) = mpsc::channel(64);
        self.alarm_subscribers.write().await.push(tx);
        rx
    }
    
    /// Subscribes to LEX events (for StreamSight integration).
    pub async fn subscribe_lex(&self) -> mpsc::Receiver<LexEvent> {
        let (tx, rx) = mpsc::channel(256);
        self.lex_subscribers.write().await.push(tx);
        rx
    }
    
    /// Gets current metrics.
    pub async fn metrics(&self) -> GatewayMetrics {
        self.metrics.read().await.clone()
    }
    
    /// Gets device status.
    pub async fn device_status(&self, device_id: &str) -> Option<DeviceState> {
        let clients = self.clients.read().await;
        clients.get(device_id).map(|c| {
            if c.is_connected() {
                DeviceState::Online
            } else {
                DeviceState::Offline
            }
        })
    }
    
    /// Manually reads a register (outside of scheduled polling).
    pub async fn read_register(
        &self,
        device_id: &str,
        address: u16,
        count: u16,
    ) -> Result<Vec<u16>> {
        let clients = self.clients.read().await;
        let client = clients.get(device_id).ok_or_else(|| {
            IndustrialError::DeviceNotFound {
                device_id: device_id.to_string(),
            }
        })?;
        
        client.read_holding_registers(0, address, count).await
    }
    
    /// Manually writes a register.
    pub async fn write_register(
        &self,
        device_id: &str,
        address: u16,
        value: u16,
    ) -> Result<()> {
        let clients = self.clients.read().await;
        let client = clients.get(device_id).ok_or_else(|| {
            IndustrialError::DeviceNotFound {
                device_id: device_id.to_string(),
            }
        })?;
        
        use crate::protocol::ModbusWriteRequest;
        client.write_single(ModbusWriteRequest {
            request_id: 0,
            register_type: RegisterType::Holding,
            address,
            values: vec![value],
        }).await
    }
    
    /// Emits a health event.
    pub async fn emit_health(&self) {
        let metrics = self.metrics.read().await;
        let uptime = self.start_time.read().await
            .map(|t| t.elapsed().as_secs())
            .unwrap_or(0);
        
        let event = GatewayHealthEvent {
            gateway_id: hex::encode(&self.config.gateway_id[..16]),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: uptime,
            devices_configured: metrics.devices_configured,
            devices_online: metrics.devices_online,
            registers_configured: self.config.registers.len() as u32,
            alarms_active: metrics.alarms_active,
            requests_total: metrics.requests_total,
            requests_failed: metrics.responses_failed,
            avg_latency_us: metrics.avg_latency_us,
            timestamp_ns: timestamp_ns(),
        };
        
        self.bridge.process(TelemetryEvent::GatewayHealth(event)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    
    fn test_config() -> GatewayConfig {
        GatewayConfig {
            gateway_id: [1u8; 32],
            name: "Test Gateway".into(),
            devices: vec![DeviceConfig {
                device_id: "test-plc".into(),
                name: "Test PLC".into(),
                ip_address: Ipv4Addr::new(127, 0, 0, 1),
                port: 5020,
                ..Default::default()
            }],
            registers: vec![RegisterConfig {
                device_id: "test-plc".into(),
                name: "temperature".into(),
                address: 100,
                ..Default::default()
            }],
            alarms: vec![],
            streamsight: Default::default(),
            settings: Default::default(),
        }
    }
    
    #[tokio::test]
    async fn test_gateway_lite_creation() {
        let gateway = GatewayLite::new(test_config()).await;
        assert!(gateway.is_ok());
        
        let gateway = gateway.unwrap();
        assert_eq!(gateway.name(), "Test Gateway");
        assert!(!gateway.is_running());
    }
    
    #[tokio::test]
    async fn test_gateway_lite_device_limit() {
        let mut config = test_config();
        config.devices = (0..11)
            .map(|i| DeviceConfig {
                device_id: format!("plc-{}", i),
                ip_address: Ipv4Addr::new(127, 0, 0, 1),
                ..Default::default()
            })
            .collect();
        
        let result = GatewayLite::new(config).await;
        assert!(matches!(result, Err(IndustrialError::LimitExceeded { .. })));
    }
}
