//! MODBUS protocol implementation.
//!
//! Implements the `circuits/industrial/modbus_tcp_client.escir.yaml` ESCIR circuit.

use crate::config::DeviceConfig;
use crate::transport::{TcpClient, TcpConfig, TcpEvent};
use crate::types::*;
use crate::{IndustrialError, Result};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, warn};

/// MODBUS TCP client.
pub struct ModbusTcpClient {
    /// Device configuration
    config: DeviceConfig,
    /// TCP transport
    tcp: Arc<TcpClient>,
    /// Transaction ID counter
    transaction_id: AtomicU16,
    /// In-flight requests: transaction_id -> (request_id, timestamp)
    inflight: RwLock<HashMap<u16, (u32, u64)>>,
    /// Event channel
    event_tx: Option<mpsc::Sender<ModbusEvent>>,
}

/// MODBUS client events for StreamSight.
#[derive(Debug, Clone)]
pub enum ModbusEvent {
    /// Request sent
    Request {
        device_id: String,
        transaction_id: u16,
        function_code: u8,
        address: u16,
        quantity: u16,
        timestamp_ns: u64,
    },
    /// Response received
    Response {
        device_id: String,
        transaction_id: u16,
        success: bool,
        latency_us: u32,
        timestamp_ns: u64,
    },
    /// Exception received
    Exception {
        device_id: String,
        transaction_id: u16,
        function_code: u8,
        exception_code: u8,
        timestamp_ns: u64,
    },
}

/// MODBUS read request.
#[derive(Debug, Clone)]
pub struct ModbusReadRequest {
    /// Application request ID
    pub request_id: u32,
    /// Register type
    pub register_type: RegisterType,
    /// Starting address
    pub address: u16,
    /// Number of registers
    pub quantity: u16,
}

/// MODBUS read response.
#[derive(Debug, Clone)]
pub struct ModbusReadResponse {
    /// Correlated request ID
    pub request_id: u32,
    /// Transaction ID
    pub transaction_id: u16,
    /// Success status
    pub success: bool,
    /// Response data (raw 16-bit values)
    pub values: Vec<u16>,
    /// Exception code (if !success)
    pub exception_code: Option<u8>,
    /// Latency in microseconds
    pub latency_us: u32,
}

/// MODBUS write request.
#[derive(Debug, Clone)]
pub struct ModbusWriteRequest {
    /// Application request ID
    pub request_id: u32,
    /// Register type (Holding or Coil)
    pub register_type: RegisterType,
    /// Starting address
    pub address: u16,
    /// Values to write
    pub values: Vec<u16>,
}

impl ModbusTcpClient {
    /// Creates a new MODBUS TCP client.
    pub fn new(config: DeviceConfig) -> Self {
        let addr: SocketAddr = format!("{}:{}", config.ip_address, config.port)
            .parse()
            .expect("Invalid address");
        
        let tcp_config = TcpConfig {
            remote_addr: addr,
            connect_timeout: Duration::from_millis(config.connect_timeout_ms as u64),
            read_timeout: Duration::from_millis(config.response_timeout_ms as u64),
            write_timeout: Duration::from_millis(config.response_timeout_ms as u64),
            reconnect_delay: Duration::from_millis(config.retry_delay_ms as u64),
            max_reconnect_attempts: config.retry_count as u32,
            ..Default::default()
        };
        
        Self {
            config,
            tcp: Arc::new(TcpClient::new(tcp_config)),
            transaction_id: AtomicU16::new(1),
            inflight: RwLock::new(HashMap::new()),
            event_tx: None,
        }
    }
    
    /// Creates a client with event channel.
    pub fn with_events(config: DeviceConfig, event_tx: mpsc::Sender<ModbusEvent>) -> Self {
        let mut client = Self::new(config);
        client.event_tx = Some(event_tx);
        client
    }
    
    /// Returns the device ID.
    pub fn device_id(&self) -> &str {
        &self.config.device_id
    }
    
    /// Connects to the device.
    pub async fn connect(&self) -> Result<()> {
        use crate::transport::Transport;
        self.tcp.connect().await
    }
    
    /// Disconnects from the device.
    pub async fn disconnect(&self) -> Result<()> {
        use crate::transport::Transport;
        self.tcp.disconnect().await
    }
    
    /// Returns whether the client is connected.
    pub fn is_connected(&self) -> bool {
        use crate::transport::Transport;
        self.tcp.is_connected()
    }
    
    /// Generates next transaction ID.
    fn next_transaction_id(&self) -> u16 {
        let id = self.transaction_id.fetch_add(1, Ordering::SeqCst);
        if id == 0 {
            self.transaction_id.fetch_add(1, Ordering::SeqCst)
        } else {
            id
        }
    }
    
    /// Builds MBAP header.
    fn build_mbap(&self, transaction_id: u16, pdu_length: usize) -> [u8; 7] {
        let length = (pdu_length + 1) as u16; // PDU + unit_id
        [
            (transaction_id >> 8) as u8,
            (transaction_id & 0xFF) as u8,
            0x00, // Protocol ID high
            0x00, // Protocol ID low
            (length >> 8) as u8,
            (length & 0xFF) as u8,
            self.config.unit_id,
        ]
    }
    
    /// Parses MBAP header from response.
    fn parse_mbap<'a>(&self, data: &'a [u8]) -> Result<(u16, u8, &'a [u8])> {
        if data.len() < 8 {
            return Err(IndustrialError::InvalidResponse {
                reason: "Response too short".into(),
            });
        }
        
        let transaction_id = ((data[0] as u16) << 8) | (data[1] as u16);
        let protocol_id = ((data[2] as u16) << 8) | (data[3] as u16);
        let length = ((data[4] as u16) << 8) | (data[5] as u16);
        let unit_id = data[6];
        
        if protocol_id != 0 {
            return Err(IndustrialError::InvalidResponse {
                reason: format!("Invalid protocol ID: {}", protocol_id),
            });
        }
        
        let pdu = &data[7..];
        Ok((transaction_id, unit_id, pdu))
    }
    
    /// Reads registers from the device.
    pub async fn read(&self, request: ModbusReadRequest) -> Result<ModbusReadResponse> {
        let transaction_id = self.next_transaction_id();
        let function_code = request.register_type.read_function_code();
        
        // Build PDU
        let pdu = [
            function_code,
            (request.address >> 8) as u8,
            (request.address & 0xFF) as u8,
            (request.quantity >> 8) as u8,
            (request.quantity & 0xFF) as u8,
        ];
        
        // Build full frame
        let mbap = self.build_mbap(transaction_id, pdu.len());
        let mut frame = Vec::with_capacity(12);
        frame.extend_from_slice(&mbap);
        frame.extend_from_slice(&pdu);
        
        // Track in-flight
        let send_time = timestamp_ns();
        self.inflight.write().await.insert(transaction_id, (request.request_id, send_time));
        
        // Emit request event
        if let Some(tx) = &self.event_tx {
            let _ = tx.send(ModbusEvent::Request {
                device_id: self.config.device_id.clone(),
                transaction_id,
                function_code,
                address: request.address,
                quantity: request.quantity,
                timestamp_ns: send_time,
            }).await;
        }
        
        // Send and receive
        let response = self.tcp.send_receive(request.request_id, &frame).await?;
        let recv_time = timestamp_ns();
        let latency_us = ((recv_time - send_time) / 1000) as u32;
        
        // Remove from in-flight
        self.inflight.write().await.remove(&transaction_id);
        
        // Parse response
        let (resp_trans_id, _unit_id, pdu) = self.parse_mbap(&response)?;
        
        // Verify transaction ID
        if resp_trans_id != transaction_id {
            return Err(IndustrialError::TransactionMismatch {
                expected: transaction_id,
                actual: resp_trans_id,
            });
        }
        
        // Check for exception
        if pdu[0] & 0x80 != 0 {
            let exception_code = pdu[1];
            
            // Emit exception event
            if let Some(tx) = &self.event_tx {
                let _ = tx.send(ModbusEvent::Exception {
                    device_id: self.config.device_id.clone(),
                    transaction_id,
                    function_code: pdu[0] & 0x7F,
                    exception_code,
                    timestamp_ns: recv_time,
                }).await;
            }
            
            return Ok(ModbusReadResponse {
                request_id: request.request_id,
                transaction_id,
                success: false,
                values: vec![],
                exception_code: Some(exception_code),
                latency_us,
            });
        }
        
        // Parse data
        let byte_count = pdu[1] as usize;
        let mut values = Vec::with_capacity(request.quantity as usize);
        
        for i in 0..(byte_count / 2) {
            let idx = 2 + i * 2;
            let value = ((pdu[idx] as u16) << 8) | (pdu[idx + 1] as u16);
            values.push(value);
        }
        
        // Emit response event
        if let Some(tx) = &self.event_tx {
            let _ = tx.send(ModbusEvent::Response {
                device_id: self.config.device_id.clone(),
                transaction_id,
                success: true,
                latency_us,
                timestamp_ns: recv_time,
            }).await;
        }
        
        Ok(ModbusReadResponse {
            request_id: request.request_id,
            transaction_id,
            success: true,
            values,
            exception_code: None,
            latency_us,
        })
    }
    
    /// Writes a single register.
    pub async fn write_single(&self, request: ModbusWriteRequest) -> Result<()> {
        if request.values.is_empty() {
            return Err(IndustrialError::InvalidConfig {
                reason: "No values to write".into(),
            });
        }
        
        let transaction_id = self.next_transaction_id();
        let function_code = match request.register_type {
            RegisterType::Holding => 0x06,
            RegisterType::Coil => 0x05,
            _ => return Err(IndustrialError::InvalidConfig {
                reason: "Cannot write to read-only register type".into(),
            }),
        };
        
        let value = if request.register_type == RegisterType::Coil {
            if request.values[0] != 0 { 0xFF00 } else { 0x0000 }
        } else {
            request.values[0]
        };
        
        // Build PDU
        let pdu = [
            function_code,
            (request.address >> 8) as u8,
            (request.address & 0xFF) as u8,
            (value >> 8) as u8,
            (value & 0xFF) as u8,
        ];
        
        let mbap = self.build_mbap(transaction_id, pdu.len());
        let mut frame = Vec::with_capacity(12);
        frame.extend_from_slice(&mbap);
        frame.extend_from_slice(&pdu);
        
        let _response = self.tcp.send_receive(request.request_id, &frame).await?;
        
        Ok(())
    }
    
    /// Writes multiple registers.
    pub async fn write_multiple(&self, request: ModbusWriteRequest) -> Result<()> {
        let transaction_id = self.next_transaction_id();
        let function_code = match request.register_type {
            RegisterType::Holding => 0x10,
            RegisterType::Coil => 0x0F,
            _ => return Err(IndustrialError::InvalidConfig {
                reason: "Cannot write to read-only register type".into(),
            }),
        };
        
        let quantity = request.values.len() as u16;
        let byte_count = (quantity * 2) as u8;
        
        // Build PDU
        let mut pdu = Vec::with_capacity(6 + request.values.len() * 2);
        pdu.push(function_code);
        pdu.push((request.address >> 8) as u8);
        pdu.push((request.address & 0xFF) as u8);
        pdu.push((quantity >> 8) as u8);
        pdu.push((quantity & 0xFF) as u8);
        pdu.push(byte_count);
        
        for value in &request.values {
            pdu.push((*value >> 8) as u8);
            pdu.push((*value & 0xFF) as u8);
        }
        
        let mbap = self.build_mbap(transaction_id, pdu.len());
        let mut frame = Vec::with_capacity(7 + pdu.len());
        frame.extend_from_slice(&mbap);
        frame.extend_from_slice(&pdu);
        
        let _response = self.tcp.send_receive(request.request_id, &frame).await?;
        
        Ok(())
    }
    
    /// Convenience method: Read holding registers.
    pub async fn read_holding_registers(
        &self,
        request_id: u32,
        address: u16,
        quantity: u16,
    ) -> Result<Vec<u16>> {
        let response = self.read(ModbusReadRequest {
            request_id,
            register_type: RegisterType::Holding,
            address,
            quantity,
        }).await?;
        
        if response.success {
            Ok(response.values)
        } else {
            Err(IndustrialError::modbus_exception(
                0x03,
                response.exception_code.unwrap_or(0),
            ))
        }
    }
    
    /// Convenience method: Read input registers.
    pub async fn read_input_registers(
        &self,
        request_id: u32,
        address: u16,
        quantity: u16,
    ) -> Result<Vec<u16>> {
        let response = self.read(ModbusReadRequest {
            request_id,
            register_type: RegisterType::Input,
            address,
            quantity,
        }).await?;
        
        if response.success {
            Ok(response.values)
        } else {
            Err(IndustrialError::modbus_exception(
                0x04,
                response.exception_code.unwrap_or(0),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    
    fn test_config() -> DeviceConfig {
        DeviceConfig {
            device_id: "test".into(),
            name: "Test Device".into(),
            ip_address: Ipv4Addr::new(127, 0, 0, 1),
            port: 5020,
            unit_id: 1,
            ..Default::default()
        }
    }
    
    #[test]
    fn test_mbap_build() {
        let client = ModbusTcpClient::new(test_config());
        let mbap = client.build_mbap(1, 5);
        
        assert_eq!(mbap[0], 0x00); // Transaction ID high
        assert_eq!(mbap[1], 0x01); // Transaction ID low
        assert_eq!(mbap[2], 0x00); // Protocol ID high
        assert_eq!(mbap[3], 0x00); // Protocol ID low
        assert_eq!(mbap[4], 0x00); // Length high
        assert_eq!(mbap[5], 0x06); // Length low (5 + 1)
        assert_eq!(mbap[6], 0x01); // Unit ID
    }
    
    #[test]
    fn test_transaction_id_wrap() {
        let client = ModbusTcpClient::new(test_config());
        
        // Should never return 0
        for _ in 0..70000 {
            let id = client.next_transaction_id();
            assert_ne!(id, 0);
        }
    }
}
