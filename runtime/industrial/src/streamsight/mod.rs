//! StreamSight integration for the industrial gateway.
//!
//! This module provides the telemetry bridge that aggregates events from
//! connection, protocol, device, and alarm subsystems into StreamSight-compatible
//! LEX streams.
//!
//! Implements `circuits/industrial/industrial_streamsight_bridge.escir.yaml`

use crate::emitter::{StreamEvent, AlarmEventOutput};
use crate::protocol::ModbusEvent;
use crate::transport::TcpEvent;
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, info, warn};

/// StreamSight bridge configuration.
#[derive(Debug, Clone)]
pub struct BridgeConfig {
    /// Gateway ID
    pub gateway_id: [u8; 32],
    /// LEX namespace
    pub namespace: String,
    /// Buffer size
    pub buffer_size: u16,
    /// Batch size
    pub batch_size: u16,
    /// Flush interval in milliseconds
    pub flush_interval_ms: u32,
    /// Minimum severity to emit (0=debug, 1=info, 2=warning, 3=error)
    pub severity_filter: u8,
    /// Sampling rate for debug events
    pub sampling_rate: f32,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            gateway_id: [0u8; 32],
            namespace: "lex://estream/sys/industrial".into(),
            buffer_size: 256,
            batch_size: 32,
            flush_interval_ms: 100,
            severity_filter: 0,
            sampling_rate: 1.0,
        }
    }
}

/// Severity levels for telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Severity {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

/// A serialized event ready for LEX emission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexEvent {
    /// Topic
    pub topic: String,
    /// Payload (serialized JSON)
    pub payload: serde_json::Value,
    /// Severity
    pub severity: u8,
    /// Timestamp
    pub timestamp_ns: u64,
    /// Sequence number
    pub sequence_number: u64,
}

/// Bridge metrics.
#[derive(Debug, Clone, Default)]
pub struct BridgeMetrics {
    /// Events received
    pub events_received: u64,
    /// Events emitted
    pub events_emitted: u64,
    /// Events filtered (severity)
    pub events_filtered: u64,
    /// Events sampled out
    pub events_sampled_out: u64,
    /// Batches sent
    pub batches_sent: u64,
    /// Bytes sent
    pub bytes_sent: u64,
}

/// Unified telemetry event from any subsystem.
#[derive(Debug, Clone)]
pub enum TelemetryEvent {
    /// TCP connection event
    TcpEvent(TcpEvent),
    /// MODBUS protocol event
    ModbusEvent(ModbusEvent),
    /// Stream event
    StreamEvent(StreamEvent),
    /// Alarm event
    AlarmEvent(AlarmEventOutput),
    /// Gateway health
    GatewayHealth(GatewayHealthEvent),
}

/// Gateway health event.
#[derive(Debug, Clone, Serialize)]
pub struct GatewayHealthEvent {
    pub gateway_id: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub devices_configured: u16,
    pub devices_online: u16,
    pub registers_configured: u32,
    pub alarms_active: u16,
    pub requests_total: u64,
    pub requests_failed: u64,
    pub avg_latency_us: u32,
    pub timestamp_ns: u64,
}

/// StreamSight bridge.
pub struct StreamSightBridge {
    config: BridgeConfig,
    /// Metrics
    metrics: RwLock<BridgeMetrics>,
    /// Sequence counter
    sequence: std::sync::atomic::AtomicU64,
    /// Output channel
    output_tx: mpsc::Sender<LexEvent>,
    /// Event buffer
    buffer: RwLock<Vec<LexEvent>>,
    /// Running flag
    running: std::sync::atomic::AtomicBool,
}

impl StreamSightBridge {
    /// Creates a new StreamSight bridge.
    pub fn new(config: BridgeConfig, output_tx: mpsc::Sender<LexEvent>) -> Self {
        Self {
            config,
            metrics: RwLock::new(BridgeMetrics::default()),
            sequence: std::sync::atomic::AtomicU64::new(1),
            output_tx,
            buffer: RwLock::new(Vec::new()),
            running: std::sync::atomic::AtomicBool::new(false),
        }
    }
    
    /// Returns the gateway ID as hex string.
    fn gateway_id_hex(&self) -> String {
        hex::encode(&self.config.gateway_id[..16])
    }
    
    /// Processes a telemetry event.
    pub async fn process(&self, event: TelemetryEvent) {
        self.metrics.write().await.events_received += 1;
        
        let (topic, payload, severity) = match &event {
            TelemetryEvent::TcpEvent(e) => self.process_tcp_event(e),
            TelemetryEvent::ModbusEvent(e) => self.process_modbus_event(e),
            TelemetryEvent::StreamEvent(e) => self.process_stream_event(e),
            TelemetryEvent::AlarmEvent(e) => self.process_alarm_event(e),
            TelemetryEvent::GatewayHealth(e) => self.process_health_event(e),
        };
        
        // Severity filter
        if (severity as u8) < self.config.severity_filter {
            self.metrics.write().await.events_filtered += 1;
            return;
        }
        
        // Sampling for debug events
        if severity == Severity::Debug && self.config.sampling_rate < 1.0 {
            let r: f32 = rand::random();
            if r > self.config.sampling_rate {
                self.metrics.write().await.events_sampled_out += 1;
                return;
            }
        }
        
        let sequence = self.sequence.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let lex_event = LexEvent {
            topic,
            payload,
            severity: severity as u8,
            timestamp_ns: timestamp_ns(),
            sequence_number: sequence,
        };
        
        // Buffer for batching
        let should_flush = {
            let mut buffer = self.buffer.write().await;
            buffer.push(lex_event);
            buffer.len() >= self.config.batch_size as usize
        };
        
        if should_flush {
            self.flush().await;
        }
    }
    
    /// Flushes the buffer.
    pub async fn flush(&self) {
        let events = {
            let mut buffer = self.buffer.write().await;
            std::mem::take(&mut *buffer)
        };
        
        if events.is_empty() {
            return;
        }
        
        let mut metrics = self.metrics.write().await;
        metrics.events_emitted += events.len() as u64;
        metrics.batches_sent += 1;
        
        for event in events {
            let bytes = serde_json::to_string(&event.payload)
                .map(|s| s.len())
                .unwrap_or(0);
            metrics.bytes_sent += bytes as u64;
            
            if self.output_tx.send(event).await.is_err() {
                warn!("StreamSight output channel closed");
                break;
            }
        }
    }
    
    fn process_tcp_event(&self, event: &TcpEvent) -> (String, serde_json::Value, Severity) {
        let topic = format!("{}/{}/connection", self.config.namespace, self.gateway_id_hex());
        
        let (payload, severity) = match event {
            TcpEvent::StateChange { old_state, new_state, timestamp_ns } => {
                (serde_json::json!({
                    "type": "state_change",
                    "old_state": format!("{:?}", old_state),
                    "new_state": format!("{:?}", new_state),
                    "timestamp_ns": timestamp_ns
                }), Severity::Info)
            }
            TcpEvent::RequestSent { request_id, bytes, timestamp_ns } => {
                (serde_json::json!({
                    "type": "request_sent",
                    "request_id": request_id,
                    "bytes": bytes,
                    "timestamp_ns": timestamp_ns
                }), Severity::Debug)
            }
            TcpEvent::ResponseReceived { request_id, bytes, latency_us, timestamp_ns } => {
                (serde_json::json!({
                    "type": "response_received",
                    "request_id": request_id,
                    "bytes": bytes,
                    "latency_us": latency_us,
                    "timestamp_ns": timestamp_ns
                }), Severity::Debug)
            }
            TcpEvent::Error { error_code, message, timestamp_ns } => {
                (serde_json::json!({
                    "type": "error",
                    "error_code": error_code,
                    "message": message,
                    "timestamp_ns": timestamp_ns
                }), Severity::Warning)
            }
        };
        
        (topic, payload, severity)
    }
    
    fn process_modbus_event(&self, event: &ModbusEvent) -> (String, serde_json::Value, Severity) {
        let topic = format!("{}/{}/protocol/modbus", self.config.namespace, self.gateway_id_hex());
        
        let (payload, severity) = match event {
            ModbusEvent::Request { device_id, transaction_id, function_code, address, quantity, timestamp_ns } => {
                (serde_json::json!({
                    "type": "request",
                    "device_id": device_id,
                    "transaction_id": transaction_id,
                    "function_code": function_code,
                    "address": address,
                    "quantity": quantity,
                    "timestamp_ns": timestamp_ns
                }), Severity::Debug)
            }
            ModbusEvent::Response { device_id, transaction_id, success, latency_us, timestamp_ns } => {
                (serde_json::json!({
                    "type": "response",
                    "device_id": device_id,
                    "transaction_id": transaction_id,
                    "success": success,
                    "latency_us": latency_us,
                    "timestamp_ns": timestamp_ns
                }), Severity::Debug)
            }
            ModbusEvent::Exception { device_id, transaction_id, function_code, exception_code, timestamp_ns } => {
                (serde_json::json!({
                    "type": "exception",
                    "device_id": device_id,
                    "transaction_id": transaction_id,
                    "function_code": function_code,
                    "exception_code": exception_code,
                    "timestamp_ns": timestamp_ns
                }), Severity::Warning)
            }
        };
        
        (topic, payload, severity)
    }
    
    fn process_stream_event(&self, event: &StreamEvent) -> (String, serde_json::Value, Severity) {
        let topic = format!(
            "{}/{}/device/{}/telemetry",
            self.config.namespace,
            self.gateway_id_hex(),
            event.device_id
        );
        
        let payload = serde_json::json!({
            "type": "value",
            "event_id": event.event_id,
            "name": event.name,
            "value": event.value,
            "unit": event.unit,
            "quality": event.quality as u8,
            "source_timestamp_ns": event.source_timestamp_ns,
            "server_timestamp_ns": event.server_timestamp_ns
        });
        
        (topic, payload, Severity::Debug)
    }
    
    fn process_alarm_event(&self, event: &AlarmEventOutput) -> (String, serde_json::Value, Severity) {
        let topic = format!("{}/{}/alarm", self.config.namespace, self.gateway_id_hex());
        
        let severity = match event.severity {
            AlarmSeverity::Info => Severity::Info,
            AlarmSeverity::Warning => Severity::Warning,
            AlarmSeverity::Critical => Severity::Error,
        };
        
        let payload = serde_json::json!({
            "type": if event.state == AlarmState::Active { "alarm_active" } else { "alarm_cleared" },
            "alarm_id": event.alarm_id,
            "name": event.name,
            "state": format!("{:?}", event.state),
            "severity": format!("{:?}", event.severity),
            "current_value": event.current_value,
            "threshold_value": event.threshold_value,
            "message": event.message,
            "timestamp_ns": event.timestamp_ns
        });
        
        (topic, payload, severity)
    }
    
    fn process_health_event(&self, event: &GatewayHealthEvent) -> (String, serde_json::Value, Severity) {
        let topic = format!("{}/{}/health", self.config.namespace, self.gateway_id_hex());
        
        let payload = serde_json::to_value(event).unwrap_or_default();
        
        (topic, payload, Severity::Info)
    }
    
    /// Gets current metrics.
    pub async fn metrics(&self) -> BridgeMetrics {
        self.metrics.read().await.clone()
    }
}

// Stub for rand since we don't want to add the full dependency
mod rand {
    pub fn random<T: Default>() -> T {
        T::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_bridge_processes_tcp_event() {
        let (tx, mut rx) = mpsc::channel(10);
        let bridge = StreamSightBridge::new(BridgeConfig::default(), tx);
        
        bridge.process(TelemetryEvent::TcpEvent(TcpEvent::StateChange {
            old_state: ConnectionState::Disconnected,
            new_state: ConnectionState::Connected,
            timestamp_ns: 12345,
        })).await;
        
        bridge.flush().await;
        
        let event = rx.try_recv();
        assert!(event.is_ok());
    }
}
