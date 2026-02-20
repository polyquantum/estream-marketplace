//! Stream emitter implementation.
//!
//! This module converts raw register values to stream events:
//! - Data type conversion and scaling
//! - Change detection
//! - Alarm evaluation
//!
//! Implements `circuits/industrial/stream_emitter.escir.yaml`

use crate::config::{RegisterConfig, AlarmConfig};
use crate::types::*;
use crate::{IndustrialError, Result};
use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, warn};

/// Emitter configuration.
#[derive(Debug, Clone)]
pub struct EmitterConfig {
    /// Gateway ID
    pub gateway_id: [u8; 32],
    /// Default emit-on-change behavior
    pub emit_on_change_only: bool,
    /// Default change threshold
    pub change_threshold: f64,
    /// Enable batching
    pub batch_enabled: bool,
    /// Batch size
    pub batch_size: u16,
    /// Batch timeout in milliseconds
    pub batch_timeout_ms: u32,
}

impl Default for EmitterConfig {
    fn default() -> Self {
        Self {
            gateway_id: [0u8; 32],
            emit_on_change_only: false,
            change_threshold: 0.0,
            batch_enabled: true,
            batch_size: 32,
            batch_timeout_ms: 100,
        }
    }
}

/// A stream event.
#[derive(Debug, Clone)]
pub struct StreamEvent {
    /// Event ID
    pub event_id: u64,
    /// Device ID
    pub device_id: String,
    /// Register name
    pub name: String,
    /// Scaled value
    pub value: f64,
    /// Engineering unit
    pub unit: String,
    /// Data quality
    pub quality: Quality,
    /// Source timestamp
    pub source_timestamp_ns: u64,
    /// Server timestamp
    pub server_timestamp_ns: u64,
    /// LEX topic
    pub topic: String,
}

/// An alarm event.
#[derive(Debug, Clone)]
pub struct AlarmEventOutput {
    /// Alarm ID
    pub alarm_id: String,
    /// Alarm name
    pub name: String,
    /// Current state
    pub state: AlarmState,
    /// Severity
    pub severity: AlarmSeverity,
    /// Current value
    pub current_value: f64,
    /// Threshold value
    pub threshold_value: f64,
    /// Message
    pub message: String,
    /// Timestamp
    pub timestamp_ns: u64,
}

/// Register mapping with runtime state.
#[derive(Debug, Clone)]
struct RegisterMapping {
    config: RegisterConfig,
    last_value: Option<f64>,
    last_raw: Option<Vec<u16>>,
}

/// Alarm state tracking.
#[derive(Debug, Clone)]
struct AlarmTracking {
    config: AlarmConfig,
    state: AlarmState,
    active_since_ns: Option<u64>,
    debounce_until_ns: Option<u64>,
}

/// Stream emitter.
pub struct StreamEmitter {
    config: EmitterConfig,
    /// Register mappings by (device_id, address)
    registers: RwLock<HashMap<(String, u16), RegisterMapping>>,
    /// Register mappings by name (for alarm lookup)
    registers_by_name: RwLock<HashMap<String, (String, u16)>>,
    /// Alarm configurations
    alarms: RwLock<HashMap<String, AlarmTracking>>,
    /// Event ID counter
    event_id: std::sync::atomic::AtomicU64,
    /// Stream event channel
    event_tx: mpsc::Sender<StreamEvent>,
    /// Alarm event channel
    alarm_tx: mpsc::Sender<AlarmEventOutput>,
}

impl StreamEmitter {
    /// Creates a new stream emitter.
    pub fn new(
        config: EmitterConfig,
        event_tx: mpsc::Sender<StreamEvent>,
        alarm_tx: mpsc::Sender<AlarmEventOutput>,
    ) -> Self {
        Self {
            config,
            registers: RwLock::new(HashMap::new()),
            registers_by_name: RwLock::new(HashMap::new()),
            alarms: RwLock::new(HashMap::new()),
            event_id: std::sync::atomic::AtomicU64::new(1),
            event_tx,
            alarm_tx,
        }
    }
    
    /// Adds a register mapping.
    pub async fn add_register(&self, config: RegisterConfig) {
        let key = (config.device_id.clone(), config.address);
        let name = config.name.clone();
        
        self.registers.write().await.insert(key.clone(), RegisterMapping {
            config,
            last_value: None,
            last_raw: None,
        });
        
        self.registers_by_name.write().await.insert(name, key);
    }
    
    /// Adds an alarm configuration.
    pub async fn add_alarm(&self, config: AlarmConfig) {
        self.alarms.write().await.insert(config.alarm_id.clone(), AlarmTracking {
            config,
            state: AlarmState::Normal,
            active_since_ns: None,
            debounce_until_ns: None,
        });
    }
    
    /// Processes raw register values.
    pub async fn process_raw(
        &self,
        device_id: &str,
        start_address: u16,
        values: &[u16],
        quality: Quality,
        source_timestamp_ns: Option<u64>,
    ) -> Result<()> {
        let server_timestamp = timestamp_ns();
        let source_timestamp = source_timestamp_ns.unwrap_or(server_timestamp);
        
        // Find matching registers
        let mut registers = self.registers.write().await;
        let alarms = self.alarms.read().await;
        
        for (i, &raw_value) in values.iter().enumerate() {
            let address = start_address + i as u16;
            let key = (device_id.to_string(), address);
            
            if let Some(mapping) = registers.get_mut(&key) {
                // Convert raw value to typed value
                let typed_value = self.convert_raw(raw_value, &mapping.config);
                
                // Apply scaling
                let scaled = typed_value.scaled(mapping.config.scale, mapping.config.offset);
                
                // Check for change
                let should_emit = if mapping.config.emit_on_change {
                    if let Some(last) = mapping.last_value {
                        let delta = (scaled - last).abs();
                        delta > mapping.config.change_threshold
                    } else {
                        true
                    }
                } else {
                    true
                };
                
                if should_emit {
                    mapping.last_value = Some(scaled);
                    
                    // Generate event
                    let event_id = self.event_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let topic = self.generate_topic(&mapping.config);
                    
                    let event = StreamEvent {
                        event_id,
                        device_id: device_id.to_string(),
                        name: mapping.config.name.clone(),
                        value: scaled,
                        unit: mapping.config.unit.clone(),
                        quality,
                        source_timestamp_ns: source_timestamp,
                        server_timestamp_ns: server_timestamp,
                        topic,
                    };
                    
                    // Send event
                    if self.event_tx.send(event).await.is_err() {
                        warn!("Event channel closed");
                    }
                    
                    // Evaluate alarms for this register
                    drop(alarms); // Release read lock before acquiring write lock
                    self.evaluate_alarms(&mapping.config.name, scaled, server_timestamp).await;
                    break; // Re-acquire alarms lock in next iteration
                }
            }
        }
        
        Ok(())
    }
    
    /// Converts raw u16 to RegisterValue based on data type.
    fn convert_raw(&self, raw: u16, config: &RegisterConfig) -> RegisterValue {
        match config.data_type {
            DataType::UInt16 => RegisterValue::U16(raw),
            DataType::Int16 => RegisterValue::I16(raw as i16),
            DataType::Boolean => RegisterValue::Bool(raw != 0),
            // For multi-word types, we'd need the full array
            _ => RegisterValue::U16(raw),
        }
    }
    
    /// Generates LEX topic for a register.
    fn generate_topic(&self, config: &RegisterConfig) -> String {
        format!(
            "lex://estream/sys/industrial/{}/device/{}/{}",
            hex::encode(&self.config.gateway_id[..8]),
            config.device_id,
            config.name
        )
    }
    
    /// Evaluates alarms for a register value.
    async fn evaluate_alarms(&self, register_name: &str, value: f64, timestamp_ns: u64) {
        let mut alarms = self.alarms.write().await;
        
        for (alarm_id, tracking) in alarms.iter_mut() {
            if tracking.config.register_name != register_name {
                continue;
            }
            
            if !tracking.config.enabled {
                continue;
            }
            
            // Check debounce
            if let Some(until) = tracking.debounce_until_ns {
                if timestamp_ns < until {
                    continue;
                }
            }
            
            // Evaluate condition
            let condition_met = self.evaluate_condition(
                value,
                &tracking.config,
                tracking.state == AlarmState::Active,
            );
            
            let old_state = tracking.state;
            let new_state = if condition_met {
                AlarmState::Active
            } else {
                AlarmState::Normal
            };
            
            // State change?
            if old_state != new_state {
                // Apply debounce
                if tracking.config.debounce_ms > 0 {
                    tracking.debounce_until_ns = Some(
                        timestamp_ns + (tracking.config.debounce_ms as u64 * 1_000_000)
                    );
                }
                
                tracking.state = new_state;
                
                if new_state == AlarmState::Active {
                    tracking.active_since_ns = Some(timestamp_ns);
                }
                
                // Emit alarm event
                let threshold = match tracking.config.condition {
                    AlarmCondition::LessThan | AlarmCondition::LessOrEqual => {
                        tracking.config.threshold_lo
                    }
                    _ => tracking.config.threshold_hi,
                };
                
                let message = if new_state == AlarmState::Active {
                    format!("{} triggered: {} {} {}",
                        tracking.config.name,
                        value,
                        condition_symbol(&tracking.config.condition),
                        threshold
                    )
                } else {
                    format!("{} cleared", tracking.config.name)
                };
                
                let alarm_event = AlarmEventOutput {
                    alarm_id: tracking.config.alarm_id.clone(),
                    name: tracking.config.name.clone(),
                    state: new_state,
                    severity: tracking.config.severity,
                    current_value: value,
                    threshold_value: threshold,
                    message,
                    timestamp_ns,
                };
                
                if self.alarm_tx.send(alarm_event).await.is_err() {
                    warn!("Alarm channel closed");
                }
            }
        }
    }
    
    /// Evaluates an alarm condition.
    fn evaluate_condition(&self, value: f64, config: &AlarmConfig, currently_active: bool) -> bool {
        // Apply hysteresis when clearing
        let threshold_hi = if currently_active {
            config.threshold_hi - config.hysteresis
        } else {
            config.threshold_hi
        };
        
        let threshold_lo = if currently_active {
            config.threshold_lo + config.hysteresis
        } else {
            config.threshold_lo
        };
        
        match config.condition {
            AlarmCondition::GreaterThan => value > threshold_hi,
            AlarmCondition::LessThan => value < threshold_lo,
            AlarmCondition::Equal => (value - config.threshold_hi).abs() < f64::EPSILON,
            AlarmCondition::NotEqual => (value - config.threshold_hi).abs() >= f64::EPSILON,
            AlarmCondition::GreaterOrEqual => value >= threshold_hi,
            AlarmCondition::LessOrEqual => value <= threshold_lo,
            AlarmCondition::Between => value >= threshold_lo && value <= threshold_hi,
            AlarmCondition::Outside => value < threshold_lo || value > threshold_hi,
        }
    }
}

fn condition_symbol(cond: &AlarmCondition) -> &'static str {
    match cond {
        AlarmCondition::GreaterThan => ">",
        AlarmCondition::LessThan => "<",
        AlarmCondition::Equal => "==",
        AlarmCondition::NotEqual => "!=",
        AlarmCondition::GreaterOrEqual => ">=",
        AlarmCondition::LessOrEqual => "<=",
        AlarmCondition::Between => "between",
        AlarmCondition::Outside => "outside",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_alarm_evaluation() {
        let (event_tx, _) = mpsc::channel(10);
        let (alarm_tx, mut alarm_rx) = mpsc::channel(10);
        
        let emitter = StreamEmitter::new(
            EmitterConfig::default(),
            event_tx,
            alarm_tx,
        );
        
        // Add register
        emitter.add_register(RegisterConfig {
            device_id: "plc1".into(),
            name: "temperature".into(),
            address: 100,
            ..Default::default()
        }).await;
        
        // Add alarm
        emitter.add_alarm(AlarmConfig {
            alarm_id: "high_temp".into(),
            register_name: "temperature".into(),
            name: "High Temperature".into(),
            condition: AlarmCondition::GreaterThan,
            threshold_hi: 80.0,
            severity: AlarmSeverity::Warning,
            enabled: true,
            ..Default::default()
        }).await;
        
        // Process value above threshold
        emitter.process_raw("plc1", 100, &[850], Quality::Good, None).await.unwrap();
        
        // Should receive alarm
        let alarm = alarm_rx.try_recv();
        assert!(alarm.is_ok());
        let alarm = alarm.unwrap();
        assert_eq!(alarm.state, AlarmState::Active);
    }
}
