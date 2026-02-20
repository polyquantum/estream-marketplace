//! # estream-industrial
//!
//! Industrial Protocol Gateway for the estream-io platform.
//!
//! This crate provides a comprehensive industrial protocol gateway with support for:
//! - MODBUS TCP/RTU
//! - OPC-UA (coming soon)
//! - DNP3 (coming soon)
//!
//! ## Architecture
//!
//! The gateway follows a layered architecture:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │              INDUSTRIAL PROTOCOL GATEWAY                         │
//! ├─────────────────────────────────────────────────────────────────┤
//! │ Layer 4: Gateway Composites                                      │
//! │   • GatewayLite, GatewayStandard, GatewayPremium                │
//! ├─────────────────────────────────────────────────────────────────┤
//! │ Layer 3: Protocol Clients                                        │
//! │   • ModbusTcpClient, ModbusRtuClient                            │
//! │   • PollScheduler, StreamEmitter                                │
//! ├─────────────────────────────────────────────────────────────────┤
//! │ Layer 2: StreamSight Integration                                 │
//! │   • IndustrialStreamSightBridge                                 │
//! │   • Telemetry events and topic routing                          │
//! ├─────────────────────────────────────────────────────────────────┤
//! │ Layer 1: Transport                                               │
//! │   • TcpClient, SerialUart                                       │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use estream_industrial::{
//!     gateway::GatewayLite,
//!     config::{GatewayConfig, DeviceConfig, RegisterConfig},
//! };
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Configure the gateway
//!     let config = GatewayConfig::builder()
//!         .gateway_id([1u8; 32])
//!         .name("Factory Floor Gateway")
//!         .add_device(DeviceConfig {
//!             device_id: "plc-01".into(),
//!             name: "Main PLC".into(),
//!             ip_address: "192.168.1.10".parse()?,
//!             port: 502,
//!             unit_id: 1,
//!             ..Default::default()
//!         })
//!         .add_register(RegisterConfig {
//!             device_id: "plc-01".into(),
//!             name: "temperature".into(),
//!             address: 100,
//!             data_type: DataType::Int16,
//!             scale: 0.1,
//!             unit: "°C".into(),
//!             poll_interval_ms: 1000,
//!             ..Default::default()
//!         })
//!         .build()?;
//!
//!     // Create and start the gateway
//!     let gateway = GatewayLite::new(config).await?;
//!     
//!     // Subscribe to stream events
//!     let mut events = gateway.subscribe_events();
//!     
//!     gateway.start().await?;
//!     
//!     // Process events
//!     while let Some(event) = events.recv().await {
//!         println!("Received: {:?}", event);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Flags
//!
//! - `modbus-tcp`: MODBUS TCP support (default)
//! - `modbus-rtu`: MODBUS RTU support (requires `serial`)
//! - `opcua`: OPC-UA client support
//! - `dnp3`: DNP3 master support
//! - `gateway-lite`: Lite tier (MODBUS TCP only)
//! - `gateway-standard`: Standard tier (TCP + RTU + OPC-UA)
//! - `gateway-premium`: Premium tier (all protocols)
//!
//! ## StreamSight Integration
//!
//! All gateway events are automatically emitted to StreamSight topics:
//!
//! - `lex://estream/sys/industrial/{gateway_id}/connection` - Connection events
//! - `lex://estream/sys/industrial/{gateway_id}/protocol/{protocol}` - Protocol events
//! - `lex://estream/sys/industrial/{gateway_id}/device/{device_id}/*` - Device telemetry
//! - `lex://estream/sys/industrial/{gateway_id}/alarm` - Alarm events
//! - `lex://estream/sys/industrial/{gateway_id}/health` - Gateway health
//!
//! ## ESCIR Alignment
//!
//! This crate implements the software reference for the ESCIR circuits defined in:
//! - `circuits/transport/tcp_client.escir.yaml`
//! - `circuits/transport/serial_uart.escir.yaml`
//! - `circuits/industrial/modbus_tcp_client.escir.yaml`
//! - `circuits/industrial/poll_scheduler.escir.yaml`
//! - `circuits/industrial/stream_emitter.escir.yaml`
//! - `circuits/industrial/industrial_streamsight_bridge.escir.yaml`

#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

pub mod config;
pub mod error;
pub mod transport;
pub mod protocol;
pub mod scheduler;
pub mod emitter;
pub mod streamsight;
pub mod gateway;
pub mod types;

// Re-exports
pub use config::{GatewayConfig, DeviceConfig, RegisterConfig, AlarmConfig};
pub use error::{IndustrialError, Result};
pub use types::*;

#[cfg(feature = "gateway-lite")]
pub use gateway::GatewayLite;

#[cfg(feature = "gateway-standard")]
pub use gateway::GatewayStandard;

#[cfg(feature = "gateway-premium")]
pub use gateway::GatewayPremium;

/// Crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// ESCIR schema version this implementation aligns with
pub const ESCIR_VERSION: &str = "0.8.0";
