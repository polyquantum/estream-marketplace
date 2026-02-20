//! Configuration types for the industrial gateway.

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

// =============================================================================
// Gateway Configuration
// =============================================================================

/// Complete gateway configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Unique gateway identifier (32 bytes)
    pub gateway_id: [u8; 32],
    
    /// Human-readable name
    pub name: String,
    
    /// Device configurations
    pub devices: Vec<DeviceConfig>,
    
    /// Register configurations
    pub registers: Vec<RegisterConfig>,
    
    /// Alarm configurations
    pub alarms: Vec<AlarmConfig>,
    
    /// StreamSight configuration
    pub streamsight: StreamSightConfig,
    
    /// Global settings
    pub settings: GatewaySettings,
}

impl GatewayConfig {
    /// Creates a new configuration builder.
    pub fn builder() -> GatewayConfigBuilder {
        GatewayConfigBuilder::default()
    }
    
    /// Validates the configuration.
    pub fn validate(&self) -> crate::Result<()> {
        // Check device limit
        if self.devices.len() > 10 {
            return Err(crate::IndustrialError::LimitExceeded {
                limit_name: "devices".into(),
                max: 10,
                requested: self.devices.len() as u32,
            });
        }
        
        // Check register limit
        if self.registers.len() > 256 {
            return Err(crate::IndustrialError::LimitExceeded {
                limit_name: "registers".into(),
                max: 256,
                requested: self.registers.len() as u32,
            });
        }
        
        // Validate all registers reference valid devices
        for reg in &self.registers {
            if !self.devices.iter().any(|d| d.device_id == reg.device_id) {
                return Err(crate::IndustrialError::DeviceNotFound {
                    device_id: reg.device_id.clone(),
                });
            }
        }
        
        Ok(())
    }
}

/// Gateway configuration builder.
#[derive(Debug, Default)]
pub struct GatewayConfigBuilder {
    gateway_id: Option<[u8; 32]>,
    name: Option<String>,
    devices: Vec<DeviceConfig>,
    registers: Vec<RegisterConfig>,
    alarms: Vec<AlarmConfig>,
    streamsight: Option<StreamSightConfig>,
    settings: Option<GatewaySettings>,
}

impl GatewayConfigBuilder {
    /// Sets the gateway ID.
    pub fn gateway_id(mut self, id: [u8; 32]) -> Self {
        self.gateway_id = Some(id);
        self
    }
    
    /// Sets the gateway name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Adds a device.
    pub fn add_device(mut self, device: DeviceConfig) -> Self {
        self.devices.push(device);
        self
    }
    
    /// Adds a register.
    pub fn add_register(mut self, register: RegisterConfig) -> Self {
        self.registers.push(register);
        self
    }
    
    /// Adds an alarm.
    pub fn add_alarm(mut self, alarm: AlarmConfig) -> Self {
        self.alarms.push(alarm);
        self
    }
    
    /// Sets StreamSight configuration.
    pub fn streamsight(mut self, config: StreamSightConfig) -> Self {
        self.streamsight = Some(config);
        self
    }
    
    /// Sets gateway settings.
    pub fn settings(mut self, settings: GatewaySettings) -> Self {
        self.settings = Some(settings);
        self
    }
    
    /// Builds the configuration.
    pub fn build(self) -> crate::Result<GatewayConfig> {
        let config = GatewayConfig {
            gateway_id: self.gateway_id.ok_or_else(|| {
                crate::IndustrialError::InvalidConfig {
                    reason: "gateway_id is required".into(),
                }
            })?,
            name: self.name.unwrap_or_else(|| "Industrial Gateway".into()),
            devices: self.devices,
            registers: self.registers,
            alarms: self.alarms,
            streamsight: self.streamsight.unwrap_or_default(),
            settings: self.settings.unwrap_or_default(),
        };
        
        config.validate()?;
        Ok(config)
    }
}

// =============================================================================
// Device Configuration
// =============================================================================

/// MODBUS device configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// Unique device identifier
    pub device_id: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Device IP address
    pub ip_address: Ipv4Addr,
    
    /// MODBUS port (default 502)
    #[serde(default = "default_modbus_port")]
    pub port: u16,
    
    /// MODBUS unit/slave ID
    #[serde(default = "default_unit_id")]
    pub unit_id: u8,
    
    /// Connection timeout in milliseconds
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_ms: u32,
    
    /// Response timeout in milliseconds
    #[serde(default = "default_response_timeout")]
    pub response_timeout_ms: u32,
    
    /// Number of retries on failure
    #[serde(default = "default_retry_count")]
    pub retry_count: u8,
    
    /// Retry delay in milliseconds
    #[serde(default = "default_retry_delay")]
    pub retry_delay_ms: u32,
    
    /// Whether device is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            device_id: String::new(),
            name: String::new(),
            ip_address: Ipv4Addr::new(127, 0, 0, 1),
            port: 502,
            unit_id: 1,
            connect_timeout_ms: 5000,
            response_timeout_ms: 1000,
            retry_count: 3,
            retry_delay_ms: 100,
            enabled: true,
        }
    }
}

// =============================================================================
// Register Configuration
// =============================================================================

/// Register mapping configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterConfig {
    /// Device this register belongs to
    pub device_id: String,
    
    /// Register name (tag name)
    pub name: String,
    
    /// Register address (0-based)
    pub address: u16,
    
    /// Register type
    #[serde(default)]
    pub register_type: RegisterType,
    
    /// Data type
    pub data_type: DataType,
    
    /// Word order for multi-word types
    #[serde(default)]
    pub word_order: WordOrder,
    
    /// Scale factor (scaled = raw * scale + offset)
    #[serde(default = "default_scale")]
    pub scale: f64,
    
    /// Offset (scaled = raw * scale + offset)
    #[serde(default)]
    pub offset: f64,
    
    /// Engineering unit
    #[serde(default)]
    pub unit: String,
    
    /// Polling interval in milliseconds
    #[serde(default = "default_poll_interval")]
    pub poll_interval_ms: u32,
    
    /// Only emit when value changes
    #[serde(default = "default_true")]
    pub emit_on_change: bool,
    
    /// Change threshold for analog values
    #[serde(default)]
    pub change_threshold: f64,
    
    /// Priority (0=low, 1=normal, 2=high, 3=critical)
    #[serde(default = "default_priority")]
    pub priority: u8,
}

impl Default for RegisterConfig {
    fn default() -> Self {
        Self {
            device_id: String::new(),
            name: String::new(),
            address: 0,
            register_type: RegisterType::Holding,
            data_type: DataType::UInt16,
            word_order: WordOrder::BigEndian,
            scale: 1.0,
            offset: 0.0,
            unit: String::new(),
            poll_interval_ms: 1000,
            emit_on_change: true,
            change_threshold: 0.0,
            priority: 1,
        }
    }
}

/// Word order for multi-word data types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum WordOrder {
    #[default]
    BigEndian = 0,
    LittleEndian = 1,
}

// =============================================================================
// Alarm Configuration
// =============================================================================

/// Alarm configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmConfig {
    /// Unique alarm identifier
    pub alarm_id: String,
    
    /// Associated register name
    pub register_name: String,
    
    /// Alarm name
    pub name: String,
    
    /// Alarm condition
    pub condition: AlarmCondition,
    
    /// Low threshold (for Between/Outside conditions)
    #[serde(default)]
    pub threshold_lo: f64,
    
    /// High threshold
    pub threshold_hi: f64,
    
    /// Hysteresis for clearing
    #[serde(default)]
    pub hysteresis: f64,
    
    /// Debounce time in milliseconds
    #[serde(default)]
    pub debounce_ms: u32,
    
    /// Alarm severity
    #[serde(default)]
    pub severity: AlarmSeverity,
    
    /// Whether alarm is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl Default for AlarmConfig {
    fn default() -> Self {
        Self {
            alarm_id: String::new(),
            register_name: String::new(),
            name: String::new(),
            condition: AlarmCondition::GreaterThan,
            threshold_lo: 0.0,
            threshold_hi: 0.0,
            hysteresis: 0.0,
            debounce_ms: 0,
            severity: AlarmSeverity::Warning,
            enabled: true,
        }
    }
}

// =============================================================================
// StreamSight Configuration
// =============================================================================

/// StreamSight telemetry configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSightConfig {
    /// Whether StreamSight is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Minimum severity to emit (0=debug, 1=info, 2=warning, 3=error)
    #[serde(default)]
    pub severity_filter: u8,
    
    /// Batch interval in milliseconds
    #[serde(default = "default_batch_interval")]
    pub batch_interval_ms: u32,
    
    /// Maximum batch size
    #[serde(default = "default_batch_size")]
    pub batch_size: u16,
    
    /// Sampling rate for debug events (0.0-1.0)
    #[serde(default = "default_sampling_rate")]
    pub sampling_rate: f32,
}

impl Default for StreamSightConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            severity_filter: 0,
            batch_interval_ms: 100,
            batch_size: 32,
            sampling_rate: 1.0,
        }
    }
}

// =============================================================================
// Gateway Settings
// =============================================================================

/// Global gateway settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewaySettings {
    /// Maximum polls per second
    #[serde(default = "default_max_polls")]
    pub max_polls_per_second: u32,
    
    /// Enable adaptive scheduling
    #[serde(default = "default_true")]
    pub adaptive_scheduling: bool,
    
    /// Backoff factor on errors
    #[serde(default = "default_backoff_factor")]
    pub backoff_factor: f32,
    
    /// Maximum backoff interval in milliseconds
    #[serde(default = "default_max_backoff")]
    pub max_backoff_interval_ms: u32,
}

impl Default for GatewaySettings {
    fn default() -> Self {
        Self {
            max_polls_per_second: 100,
            adaptive_scheduling: true,
            backoff_factor: 1.5,
            max_backoff_interval_ms: 60000,
        }
    }
}

// =============================================================================
// Default Value Functions
// =============================================================================

fn default_modbus_port() -> u16 { 502 }
fn default_unit_id() -> u8 { 1 }
fn default_connect_timeout() -> u32 { 5000 }
fn default_response_timeout() -> u32 { 1000 }
fn default_retry_count() -> u8 { 3 }
fn default_retry_delay() -> u32 { 100 }
fn default_true() -> bool { true }
fn default_scale() -> f64 { 1.0 }
fn default_poll_interval() -> u32 { 1000 }
fn default_priority() -> u8 { 1 }
fn default_batch_interval() -> u32 { 100 }
fn default_batch_size() -> u16 { 32 }
fn default_sampling_rate() -> f32 { 1.0 }
fn default_max_polls() -> u32 { 100 }
fn default_backoff_factor() -> f32 { 1.5 }
fn default_max_backoff() -> u32 { 60000 }

impl Default for RegisterType {
    fn default() -> Self {
        Self::Holding
    }
}

impl Default for AlarmSeverity {
    fn default() -> Self {
        Self::Warning
    }
}
