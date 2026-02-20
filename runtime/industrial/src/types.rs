//! Common types for the industrial gateway.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// =============================================================================
// Data Types
// =============================================================================

/// Supported register data types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum DataType {
    /// Unsigned 16-bit integer
    UInt16 = 0,
    /// Signed 16-bit integer
    Int16 = 1,
    /// Unsigned 32-bit integer (2 registers)
    UInt32 = 2,
    /// Signed 32-bit integer (2 registers)
    Int32 = 3,
    /// 32-bit floating point (2 registers)
    Float32 = 4,
    /// 64-bit floating point (4 registers)
    Float64 = 5,
    /// Boolean (single bit)
    Boolean = 6,
    /// String (multiple registers)
    String = 7,
}

impl DataType {
    /// Returns the number of 16-bit registers required for this data type.
    pub fn word_count(&self) -> u16 {
        match self {
            Self::UInt16 | Self::Int16 | Self::Boolean => 1,
            Self::UInt32 | Self::Int32 | Self::Float32 => 2,
            Self::Float64 => 4,
            Self::String => 16, // Default string length
        }
    }
}

/// Register types in MODBUS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RegisterType {
    /// Holding registers (read/write, FC 03/06/16)
    Holding = 0,
    /// Input registers (read-only, FC 04)
    Input = 1,
    /// Coils (read/write bits, FC 01/05/15)
    Coil = 2,
    /// Discrete inputs (read-only bits, FC 02)
    Discrete = 3,
}

impl RegisterType {
    /// Returns the MODBUS function code for reading this register type.
    pub fn read_function_code(&self) -> u8 {
        match self {
            Self::Holding => 0x03,
            Self::Input => 0x04,
            Self::Coil => 0x01,
            Self::Discrete => 0x02,
        }
    }
    
    /// Returns the MODBUS function code for writing this register type (None if read-only).
    pub fn write_function_code(&self) -> Option<u8> {
        match self {
            Self::Holding => Some(0x06), // Single, 0x10 for multiple
            Self::Coil => Some(0x05),    // Single, 0x0F for multiple
            Self::Input | Self::Discrete => None,
        }
    }
}

// =============================================================================
// Connection State
// =============================================================================

/// TCP connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ConnectionState {
    /// Not connected
    Disconnected = 0,
    /// Connection in progress
    Connecting = 1,
    /// Connected and ready
    Connected = 2,
    /// Attempting to reconnect
    Reconnecting = 3,
    /// Error state
    Error = 4,
}

/// Device status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum DeviceState {
    /// Status unknown
    Unknown = 0,
    /// Device is online and responding
    Online = 1,
    /// Device is offline
    Offline = 2,
    /// Device has error
    Error = 3,
}

// =============================================================================
// Alarm Types
// =============================================================================

/// Alarm condition operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum AlarmCondition {
    /// Value > threshold
    GreaterThan = 0,
    /// Value < threshold
    LessThan = 1,
    /// Value == threshold
    Equal = 2,
    /// Value != threshold
    NotEqual = 3,
    /// Value >= threshold
    GreaterOrEqual = 4,
    /// Value <= threshold
    LessOrEqual = 5,
    /// threshold_lo <= Value <= threshold_hi
    Between = 6,
    /// Value < threshold_lo OR Value > threshold_hi
    Outside = 7,
}

/// Alarm severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum AlarmSeverity {
    /// Informational
    Info = 0,
    /// Warning
    Warning = 1,
    /// Critical
    Critical = 2,
}

/// Alarm state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum AlarmState {
    /// Normal (not active)
    Normal = 0,
    /// Active (condition met)
    Active = 1,
    /// Acknowledged by operator
    Acknowledged = 2,
    /// Shelved (temporarily suppressed)
    Shelved = 3,
}

// =============================================================================
// Quality
// =============================================================================

/// OPC-style data quality codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Quality {
    /// Value is good
    Good = 0,
    /// Value overridden locally
    GoodLocalOverride = 1,
    /// Value is uncertain
    Uncertain = 64,
    /// Last known good value
    UncertainLastUsable = 65,
    /// Value is bad
    Bad = 192,
    /// Configuration error
    BadConfigError = 193,
    /// Not connected to device
    BadNotConnected = 194,
    /// Device failure
    BadDeviceFailure = 195,
    /// Sensor failure
    BadSensorFailure = 196,
    /// Communication failure
    BadCommFailure = 197,
    /// Out of service
    BadOutOfService = 198,
}

impl Quality {
    /// Returns true if quality is good.
    pub fn is_good(&self) -> bool {
        (*self as u8) < 64
    }
    
    /// Returns true if quality is uncertain.
    pub fn is_uncertain(&self) -> bool {
        let code = *self as u8;
        code >= 64 && code < 192
    }
    
    /// Returns true if quality is bad.
    pub fn is_bad(&self) -> bool {
        (*self as u8) >= 192
    }
}

// =============================================================================
// Value Types
// =============================================================================

/// A register value that can hold different data types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegisterValue {
    /// Unsigned 16-bit
    U16(u16),
    /// Signed 16-bit
    I16(i16),
    /// Unsigned 32-bit
    U32(u32),
    /// Signed 32-bit
    I32(i32),
    /// 32-bit float
    F32(f32),
    /// 64-bit float
    F64(f64),
    /// Boolean
    Bool(bool),
    /// String
    String(String),
}

impl RegisterValue {
    /// Converts to f64 for scaling and comparison.
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::U16(v) => *v as f64,
            Self::I16(v) => *v as f64,
            Self::U32(v) => *v as f64,
            Self::I32(v) => *v as f64,
            Self::F32(v) => *v as f64,
            Self::F64(v) => *v,
            Self::Bool(v) => if *v { 1.0 } else { 0.0 },
            Self::String(_) => f64::NAN,
        }
    }
    
    /// Applies scale and offset: result = value * scale + offset
    pub fn scaled(&self, scale: f64, offset: f64) -> f64 {
        self.as_f64() * scale + offset
    }
}

// =============================================================================
// Metrics
// =============================================================================

/// Connection metrics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionMetrics {
    /// Bytes sent
    pub bytes_sent: u64,
    /// Bytes received
    pub bytes_received: u64,
    /// Packets sent
    pub packets_sent: u64,
    /// Packets received
    pub packets_received: u64,
    /// Retransmissions
    pub retransmissions: u32,
    /// Average RTT in microseconds
    pub avg_rtt_us: u32,
}

/// Gateway metrics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GatewayMetrics {
    /// Total requests sent
    pub requests_total: u64,
    /// Successful responses
    pub responses_success: u64,
    /// Failed responses
    pub responses_failed: u64,
    /// Timeouts
    pub timeouts: u64,
    /// Average latency in microseconds
    pub avg_latency_us: u32,
    /// Max latency in microseconds
    pub max_latency_us: u32,
    /// P99 latency in microseconds
    pub p99_latency_us: u32,
    /// Active alarms
    pub alarms_active: u16,
    /// Devices online
    pub devices_online: u16,
    /// Devices configured
    pub devices_configured: u16,
}

// =============================================================================
// Time Utilities
// =============================================================================

/// Returns current timestamp in nanoseconds since Unix epoch.
pub fn timestamp_ns() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_data_type_word_count() {
        assert_eq!(DataType::UInt16.word_count(), 1);
        assert_eq!(DataType::Float32.word_count(), 2);
        assert_eq!(DataType::Float64.word_count(), 4);
    }
    
    #[test]
    fn test_register_value_scaling() {
        let val = RegisterValue::I16(250);
        assert_eq!(val.scaled(0.1, 0.0), 25.0);
        assert_eq!(val.scaled(0.1, 5.0), 30.0);
    }
    
    #[test]
    fn test_quality() {
        assert!(Quality::Good.is_good());
        assert!(Quality::Uncertain.is_uncertain());
        assert!(Quality::BadCommFailure.is_bad());
    }
}
