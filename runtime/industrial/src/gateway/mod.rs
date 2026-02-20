//! Gateway implementations.
//!
//! This module provides the complete gateway composites:
//! - [`GatewayLite`]: Free tier with MODBUS TCP only
//! - [`GatewayStandard`]: Commercial tier with TCP/RTU + OPC-UA
//! - [`GatewayPremium`]: Enterprise tier with all protocols

mod lite;

pub use lite::GatewayLite;

// Stub types for higher tiers
#[cfg(feature = "gateway-standard")]
pub struct GatewayStandard;

#[cfg(feature = "gateway-premium")]
pub struct GatewayPremium;
