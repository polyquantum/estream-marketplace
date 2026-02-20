//! TCP client implementation.
//!
//! Implements the `circuits/transport/tcp_client.escir.yaml` ESCIR circuit.

use crate::{ConnectionState, ConnectionMetrics, IndustrialError, Result};
use crate::types::timestamp_ns;
use async_trait::async_trait;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock, mpsc};
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

/// TCP client configuration.
#[derive(Debug, Clone)]
pub struct TcpConfig {
    /// Remote address
    pub remote_addr: SocketAddr,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Read timeout
    pub read_timeout: Duration,
    /// Write timeout
    pub write_timeout: Duration,
    /// Keepalive interval (None = disabled)
    pub keepalive_interval: Option<Duration>,
    /// Reconnect delay
    pub reconnect_delay: Duration,
    /// Maximum reconnection attempts (0 = infinite)
    pub max_reconnect_attempts: u32,
    /// TCP_NODELAY
    pub tcp_nodelay: bool,
}

impl Default for TcpConfig {
    fn default() -> Self {
        Self {
            remote_addr: "127.0.0.1:502".parse().unwrap(),
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(3),
            write_timeout: Duration::from_secs(3),
            keepalive_interval: Some(Duration::from_secs(30)),
            reconnect_delay: Duration::from_secs(1),
            max_reconnect_attempts: 10,
            tcp_nodelay: true,
        }
    }
}

/// TCP client with automatic reconnection.
pub struct TcpClient {
    config: TcpConfig,
    state: RwLock<ConnectionState>,
    stream: Mutex<Option<TcpStream>>,
    metrics: RwLock<ConnectionMetrics>,
    reconnect_count: RwLock<u32>,
    event_tx: Option<mpsc::Sender<TcpEvent>>,
}

/// TCP client events for StreamSight.
#[derive(Debug, Clone)]
pub enum TcpEvent {
    /// Connection state changed
    StateChange {
        old_state: ConnectionState,
        new_state: ConnectionState,
        timestamp_ns: u64,
    },
    /// Request sent
    RequestSent {
        request_id: u32,
        bytes: usize,
        timestamp_ns: u64,
    },
    /// Response received
    ResponseReceived {
        request_id: u32,
        bytes: usize,
        latency_us: u32,
        timestamp_ns: u64,
    },
    /// Error occurred
    Error {
        error_code: u16,
        message: String,
        timestamp_ns: u64,
    },
}

impl TcpClient {
    /// Creates a new TCP client.
    pub fn new(config: TcpConfig) -> Self {
        Self {
            config,
            state: RwLock::new(ConnectionState::Disconnected),
            stream: Mutex::new(None),
            metrics: RwLock::new(ConnectionMetrics::default()),
            reconnect_count: RwLock::new(0),
            event_tx: None,
        }
    }
    
    /// Creates a TCP client with event channel for StreamSight.
    pub fn with_events(config: TcpConfig, event_tx: mpsc::Sender<TcpEvent>) -> Self {
        Self {
            config,
            state: RwLock::new(ConnectionState::Disconnected),
            stream: Mutex::new(None),
            metrics: RwLock::new(ConnectionMetrics::default()),
            reconnect_count: RwLock::new(0),
            event_tx: Some(event_tx),
        }
    }
    
    /// Sets the connection state and emits event.
    async fn set_state(&self, new_state: ConnectionState) {
        let old_state = {
            let mut state = self.state.write().await;
            let old = *state;
            *state = new_state;
            old
        };
        
        if old_state != new_state {
            debug!("TCP state: {:?} -> {:?}", old_state, new_state);
            if let Some(tx) = &self.event_tx {
                let _ = tx.send(TcpEvent::StateChange {
                    old_state,
                    new_state,
                    timestamp_ns: timestamp_ns(),
                }).await;
            }
        }
    }
    
    /// Emits an error event.
    async fn emit_error(&self, error: &IndustrialError) {
        if let Some(tx) = &self.event_tx {
            let _ = tx.send(TcpEvent::Error {
                error_code: error.error_code(),
                message: error.to_string(),
                timestamp_ns: timestamp_ns(),
            }).await;
        }
    }
    
    /// Attempts to reconnect with backoff.
    async fn reconnect(&self) -> Result<()> {
        let max_attempts = self.config.max_reconnect_attempts;
        let mut attempts = 0u32;
        
        self.set_state(ConnectionState::Reconnecting).await;
        
        loop {
            attempts += 1;
            *self.reconnect_count.write().await = attempts;
            
            info!(
                "Reconnection attempt {} to {}",
                attempts, self.config.remote_addr
            );
            
            match self.try_connect().await {
                Ok(()) => {
                    *self.reconnect_count.write().await = 0;
                    return Ok(());
                }
                Err(e) => {
                    warn!("Reconnection failed: {}", e);
                    
                    if max_attempts > 0 && attempts >= max_attempts {
                        error!("Max reconnection attempts ({}) reached", max_attempts);
                        self.set_state(ConnectionState::Error).await;
                        return Err(e);
                    }
                    
                    // Exponential backoff
                    let delay = self.config.reconnect_delay * (1.5f64.powi(attempts as i32 - 1) as u32);
                    let delay = delay.min(Duration::from_secs(30));
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
    
    /// Internal connect without state management.
    async fn try_connect(&self) -> Result<()> {
        let connect_fut = TcpStream::connect(self.config.remote_addr);
        
        let stream = timeout(self.config.connect_timeout, connect_fut)
            .await
            .map_err(|_| IndustrialError::ConnectionTimeout {
                address: self.config.remote_addr.to_string(),
                timeout_ms: self.config.connect_timeout.as_millis() as u32,
            })?
            .map_err(|e| IndustrialError::ConnectionFailed {
                address: self.config.remote_addr.to_string(),
                reason: e.to_string(),
            })?;
        
        // Configure socket
        stream.set_nodelay(self.config.tcp_nodelay)?;
        
        *self.stream.lock().await = Some(stream);
        self.set_state(ConnectionState::Connected).await;
        
        info!("Connected to {}", self.config.remote_addr);
        Ok(())
    }
    
    /// Sends data and receives response.
    pub async fn send_receive(&self, request_id: u32, data: &[u8]) -> Result<Vec<u8>> {
        let send_time = timestamp_ns();
        
        // Send
        {
            let mut stream_guard = self.stream.lock().await;
            let stream = stream_guard.as_mut().ok_or(IndustrialError::NotConnected {
                device_id: self.config.remote_addr.to_string(),
            })?;
            
            timeout(self.config.write_timeout, stream.write_all(data))
                .await
                .map_err(|_| IndustrialError::ResponseTimeout { transaction_id: request_id })?
                .map_err(|e| IndustrialError::ConnectionReset {
                    address: self.config.remote_addr.to_string(),
                })?;
        }
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.bytes_sent += data.len() as u64;
            metrics.packets_sent += 1;
        }
        
        // Emit send event
        if let Some(tx) = &self.event_tx {
            let _ = tx.send(TcpEvent::RequestSent {
                request_id,
                bytes: data.len(),
                timestamp_ns: send_time,
            }).await;
        }
        
        // Receive response
        let mut buffer = vec![0u8; 4096];
        let n = {
            let mut stream_guard = self.stream.lock().await;
            let stream = stream_guard.as_mut().ok_or(IndustrialError::NotConnected {
                device_id: self.config.remote_addr.to_string(),
            })?;
            
            timeout(self.config.read_timeout, stream.read(&mut buffer))
                .await
                .map_err(|_| IndustrialError::ResponseTimeout { transaction_id: request_id })?
                .map_err(|e| IndustrialError::ConnectionReset {
                    address: self.config.remote_addr.to_string(),
                })?
        };
        
        let recv_time = timestamp_ns();
        let latency_us = ((recv_time - send_time) / 1000) as u32;
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.bytes_received += n as u64;
            metrics.packets_received += 1;
            // Simple moving average for RTT
            metrics.avg_rtt_us = (metrics.avg_rtt_us * 7 + latency_us) / 8;
        }
        
        // Emit receive event
        if let Some(tx) = &self.event_tx {
            let _ = tx.send(TcpEvent::ResponseReceived {
                request_id,
                bytes: n,
                latency_us,
                timestamp_ns: recv_time,
            }).await;
        }
        
        buffer.truncate(n);
        Ok(buffer)
    }
    
    /// Returns current metrics.
    pub async fn metrics(&self) -> ConnectionMetrics {
        self.metrics.read().await.clone()
    }
}

#[async_trait]
impl super::Transport for TcpClient {
    async fn send(&self, data: &[u8], expect_response: bool) -> Result<Option<Vec<u8>>> {
        if expect_response {
            let response = self.send_receive(0, data).await?;
            Ok(Some(response))
        } else {
            let mut stream_guard = self.stream.lock().await;
            let stream = stream_guard.as_mut().ok_or(IndustrialError::NotConnected {
                device_id: self.config.remote_addr.to_string(),
            })?;
            
            timeout(self.config.write_timeout, stream.write_all(data))
                .await
                .map_err(|_| IndustrialError::ResponseTimeout { transaction_id: 0 })?
                .map_err(|_| IndustrialError::ConnectionReset {
                    address: self.config.remote_addr.to_string(),
                })?;
            
            Ok(None)
        }
    }
    
    fn state(&self) -> ConnectionState {
        // Use try_read to avoid blocking
        self.state.try_read().map(|s| *s).unwrap_or(ConnectionState::Disconnected)
    }
    
    async fn connect(&self) -> Result<()> {
        self.set_state(ConnectionState::Connecting).await;
        
        match self.try_connect().await {
            Ok(()) => Ok(()),
            Err(e) => {
                self.emit_error(&e).await;
                // Attempt reconnection
                self.reconnect().await
            }
        }
    }
    
    async fn disconnect(&self) -> Result<()> {
        let mut stream_guard = self.stream.lock().await;
        if let Some(stream) = stream_guard.take() {
            drop(stream);
        }
        self.set_state(ConnectionState::Disconnected).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tcp_client_state() {
        let config = TcpConfig::default();
        let client = TcpClient::new(config);
        
        assert_eq!(client.state(), ConnectionState::Disconnected);
    }
}
