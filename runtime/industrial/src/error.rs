//! Error types for the industrial gateway.

use thiserror::Error;

/// Result type alias for industrial operations.
pub type Result<T> = std::result::Result<T, IndustrialError>;

/// Industrial gateway error types.
#[derive(Error, Debug)]
pub enum IndustrialError {
    // =========================================================================
    // Connection Errors
    // =========================================================================
    
    /// TCP connection failed
    #[error("Connection failed to {address}: {reason}")]
    ConnectionFailed {
        address: String,
        reason: String,
    },
    
    /// Connection timeout
    #[error("Connection timeout to {address} after {timeout_ms}ms")]
    ConnectionTimeout {
        address: String,
        timeout_ms: u32,
    },
    
    /// Connection refused
    #[error("Connection refused by {address}")]
    ConnectionRefused {
        address: String,
    },
    
    /// Connection reset
    #[error("Connection reset by {address}")]
    ConnectionReset {
        address: String,
    },
    
    /// Not connected
    #[error("Not connected to device {device_id}")]
    NotConnected {
        device_id: String,
    },

    // =========================================================================
    // Protocol Errors
    // =========================================================================
    
    /// MODBUS exception response
    #[error("MODBUS exception: {exception_code} - {message}")]
    ModbusException {
        function_code: u8,
        exception_code: u8,
        message: String,
    },
    
    /// Response timeout
    #[error("Response timeout for transaction {transaction_id}")]
    ResponseTimeout {
        transaction_id: u32,
    },
    
    /// Invalid response
    #[error("Invalid response: {reason}")]
    InvalidResponse {
        reason: String,
    },
    
    /// Transaction ID mismatch
    #[error("Transaction ID mismatch: expected {expected}, got {actual}")]
    TransactionMismatch {
        expected: u16,
        actual: u16,
    },
    
    /// CRC error (for RTU)
    #[error("CRC error: expected {expected:04X}, got {actual:04X}")]
    CrcError {
        expected: u16,
        actual: u16,
    },

    // =========================================================================
    // Configuration Errors
    // =========================================================================
    
    /// Invalid configuration
    #[error("Invalid configuration: {reason}")]
    InvalidConfig {
        reason: String,
    },
    
    /// Device not found
    #[error("Device not found: {device_id}")]
    DeviceNotFound {
        device_id: String,
    },
    
    /// Register not found
    #[error("Register not found: {name} on device {device_id}")]
    RegisterNotFound {
        device_id: String,
        name: String,
    },
    
    /// Limit exceeded
    #[error("Limit exceeded: {limit_name} (max: {max}, requested: {requested})")]
    LimitExceeded {
        limit_name: String,
        max: u32,
        requested: u32,
    },

    // =========================================================================
    // Serial Errors
    // =========================================================================
    
    /// Serial port error
    #[error("Serial port error on {port}: {reason}")]
    SerialError {
        port: String,
        reason: String,
    },
    
    /// Framing error
    #[error("Framing error on {port}")]
    FramingError {
        port: String,
    },
    
    /// Parity error
    #[error("Parity error on {port}")]
    ParityError {
        port: String,
    },

    // =========================================================================
    // Gateway Errors
    // =========================================================================
    
    /// Gateway not running
    #[error("Gateway is not running")]
    GatewayNotRunning,
    
    /// Gateway already running
    #[error("Gateway is already running")]
    GatewayAlreadyRunning,
    
    /// Shutdown error
    #[error("Gateway shutdown error: {reason}")]
    ShutdownError {
        reason: String,
    },

    // =========================================================================
    // Internal Errors
    // =========================================================================
    
    /// Internal error
    #[error("Internal error: {reason}")]
    Internal {
        reason: String,
    },
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Channel send error
    #[error("Channel send error")]
    ChannelSend,
    
    /// Channel receive error
    #[error("Channel receive error")]
    ChannelReceive,
}

impl IndustrialError {
    /// Returns the error code for StreamSight telemetry.
    pub fn error_code(&self) -> u16 {
        match self {
            // Connection errors: 1xx
            Self::ConnectionFailed { .. } => 100,
            Self::ConnectionTimeout { .. } => 101,
            Self::ConnectionRefused { .. } => 102,
            Self::ConnectionReset { .. } => 103,
            Self::NotConnected { .. } => 104,
            
            // Protocol errors: 2xx
            Self::ModbusException { exception_code, .. } => 200 + *exception_code as u16,
            Self::ResponseTimeout { .. } => 210,
            Self::InvalidResponse { .. } => 211,
            Self::TransactionMismatch { .. } => 212,
            Self::CrcError { .. } => 213,
            
            // Configuration errors: 3xx
            Self::InvalidConfig { .. } => 300,
            Self::DeviceNotFound { .. } => 301,
            Self::RegisterNotFound { .. } => 302,
            Self::LimitExceeded { .. } => 303,
            
            // Serial errors: 4xx
            Self::SerialError { .. } => 400,
            Self::FramingError { .. } => 401,
            Self::ParityError { .. } => 402,
            
            // Gateway errors: 5xx
            Self::GatewayNotRunning => 500,
            Self::GatewayAlreadyRunning => 501,
            Self::ShutdownError { .. } => 502,
            
            // Internal errors: 9xx
            Self::Internal { .. } => 900,
            Self::Io(_) => 901,
            Self::ChannelSend => 902,
            Self::ChannelReceive => 903,
        }
    }
    
    /// Returns whether this error is recoverable.
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ConnectionTimeout { .. } => true,
            Self::ConnectionReset { .. } => true,
            Self::ResponseTimeout { .. } => true,
            Self::ModbusException { exception_code, .. } => {
                // Exception codes 5 (Acknowledge) and 6 (Busy) are recoverable
                *exception_code == 5 || *exception_code == 6
            }
            Self::CrcError { .. } => true,
            Self::FramingError { .. } => true,
            Self::ParityError { .. } => true,
            _ => false,
        }
    }
    
    /// Creates a MODBUS exception error from exception code.
    pub fn modbus_exception(function_code: u8, exception_code: u8) -> Self {
        let message = match exception_code {
            1 => "Illegal Function",
            2 => "Illegal Data Address",
            3 => "Illegal Data Value",
            4 => "Slave Device Failure",
            5 => "Acknowledge",
            6 => "Slave Device Busy",
            _ => "Unknown Exception",
        };
        
        Self::ModbusException {
            function_code,
            exception_code,
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_codes() {
        let err = IndustrialError::ConnectionTimeout {
            address: "192.168.1.10:502".into(),
            timeout_ms: 5000,
        };
        assert_eq!(err.error_code(), 101);
        
        let err = IndustrialError::modbus_exception(0x03, 0x02);
        assert_eq!(err.error_code(), 202);
    }
    
    #[test]
    fn test_recoverable() {
        let err = IndustrialError::ResponseTimeout { transaction_id: 1 };
        assert!(err.is_recoverable());
        
        let err = IndustrialError::DeviceNotFound { device_id: "plc-01".into() };
        assert!(!err.is_recoverable());
    }
}
