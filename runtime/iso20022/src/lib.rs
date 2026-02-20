//! # estream-iso20022
//!
//! ISO 20022 message types and parsing for eStream CBDC integration.
//!
//! This crate provides:
//! - Rust types for ISO 20022 messages (pacs.008, pacs.002, camt.053, camt.052)
//! - XML parsing and generation
//! - Integration with FPGA parser via estream-fpga-bridge
//! - ESF (eStream Format) conversion
//!
//! ## Supported Messages
//!
//! | Message    | Description                      | Status |
//! |------------|----------------------------------|--------|
//! | pacs.008   | FIToFICustomerCreditTransfer    | ✓      |
//! | pacs.002   | FIToFIPaymentStatusReport       | ✓      |
//! | camt.053   | BankToCustomerStatement         | ✓      |
//! | camt.052   | BankToCustomerAccountReport     | ✓      |
//!
//! ## Example
//!
//! ```rust,ignore
//! use estream_iso20022::{Pacs008, CreditTransferTransaction};
//!
//! let msg = Pacs008::parse_xml(xml_bytes)?;
//! let esf = msg.to_esf()?;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod error;
pub mod messages;
pub mod types;
pub mod esf;
pub mod schema;

#[cfg(feature = "fpga")]
pub mod fpga;

pub use error::{Error, Result};
pub use messages::*;
pub use types::*;

/// ISO 20022 message type codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum MessageType {
    /// pacs.008 - FIToFICustomerCreditTransfer
    Pacs008 = 0x0008,
    /// pacs.002 - FIToFIPaymentStatusReport
    Pacs002 = 0x0002,
    /// camt.053 - BankToCustomerStatement
    Camt053 = 0x0053,
    /// camt.052 - BankToCustomerAccountReport
    Camt052 = 0x0052,
}

impl MessageType {
    /// Get the message type from a u16 code
    pub fn from_code(code: u16) -> Option<Self> {
        match code {
            0x0008 => Some(Self::Pacs008),
            0x0002 => Some(Self::Pacs002),
            0x0053 => Some(Self::Camt053),
            0x0052 => Some(Self::Camt052),
            _ => None,
        }
    }

    /// Get the ISO 20022 message identifier string
    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Pacs008 => "pacs.008.001.08",
            Self::Pacs002 => "pacs.002.001.10",
            Self::Camt053 => "camt.053.001.08",
            Self::Camt052 => "camt.052.001.08",
        }
    }

    /// Get the root element name
    pub fn root_element(&self) -> &'static str {
        match self {
            Self::Pacs008 => "FIToFICstmrCdtTrf",
            Self::Pacs002 => "FIToFIPmtStsRpt",
            Self::Camt053 => "BkToCstmrStmt",
            Self::Camt052 => "BkToCstmrAcctRpt",
        }
    }
}

/// Privacy tier for ESF fields (matches FPGA schema_rom.v)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrivacyTier {
    /// Visible on ledger
    Public = 0,
    /// Visible to counterparties only
    Restricted = 1,
    /// Visible to sender/receiver only
    Private = 2,
    /// Requires stealth address
    Stealth = 3,
    /// ML-KEM encrypted
    Encrypted = 4,
}

/// ESF field type codes (matches FPGA field_extractor.v)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EsfFieldType {
    None = 0x00,
    String = 0x01,
    U8 = 0x02,
    U16 = 0x03,
    U32 = 0x04,
    U64 = 0x05,
    U128 = 0x06,
    I64 = 0x07,
    Bytes = 0x08,
    Date = 0x09,
    DateTime = 0x0A,
    Decimal = 0x0B,
    Bic = 0x0C,
    Iban = 0x0D,
    Currency = 0x0E,
    Enum = 0x0F,
}
