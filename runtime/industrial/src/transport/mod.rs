//! Transport layer implementations.
//!
//! This module provides transport-level abstractions for industrial protocols:
//! - [`TcpClient`]: Generic TCP client with auto-reconnect
//! - [`SerialUart`]: Serial UART for RS-232/RS-485 (requires `serial` feature)
//!
//! These implementations align with the ESCIR circuits:
//! - `circuits/transport/tcp_client.escir.yaml`
//! - `circuits/transport/serial_uart.escir.yaml`

mod tcp;

pub use tcp::*;

#[cfg(feature = "serial")]
mod serial;

#[cfg(feature = "serial")]
pub use serial::*;

use async_trait::async_trait;

/// Transport trait for generic data exchange.
#[async_trait]
pub trait Transport: Send + Sync {
    /// Sends data and optionally waits for a response.
    async fn send(&self, data: &[u8], expect_response: bool) -> crate::Result<Option<Vec<u8>>>;
    
    /// Returns the current connection state.
    fn state(&self) -> crate::ConnectionState;
    
    /// Returns whether the transport is connected.
    fn is_connected(&self) -> bool {
        self.state() == crate::ConnectionState::Connected
    }
    
    /// Connects to the remote endpoint.
    async fn connect(&self) -> crate::Result<()>;
    
    /// Disconnects from the remote endpoint.
    async fn disconnect(&self) -> crate::Result<()>;
}
