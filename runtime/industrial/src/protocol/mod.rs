//! Protocol implementations.
//!
//! This module provides protocol-level implementations:
//! - [`ModbusTcpClient`]: MODBUS TCP master
//! - [`ModbusRtuClient`]: MODBUS RTU master (requires `serial` feature)
//!
//! These implementations align with the ESCIR circuits:
//! - `circuits/industrial/modbus_tcp_client.escir.yaml`

mod modbus;

pub use modbus::*;
