//! pacs.002 - FIToFIPaymentStatusReport message.

use alloc::string::String;
use alloc::vec::Vec;
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::*;
use crate::{Error, Result};

/// pacs.002 - FI to FI Payment Status Report
///
/// This message is sent by an instructed agent to the previous party in the
/// payment chain to report on the status of a payment instruction.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pacs002 {
    /// Group header
    pub group_header: StatusReportGroupHeader,
    /// Transaction information and status
    pub transaction_info_and_status: Vec<TransactionInfoAndStatus>,
}

impl Pacs002 {
    /// Message identifier
    pub const MESSAGE_ID: &'static str = "pacs.002.001.10";

    /// Create a status response for a pacs.008 message
    pub fn create_response(
        original_msg_id: &str,
        original_instruction_id: Option<&str>,
        status: TransactionStatus,
        status_reason: Option<StatusReason>,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            group_header: StatusReportGroupHeader {
                message_id: format!("PACS002-{}", now.timestamp_millis()),
                creation_date_time: now,
            },
            transaction_info_and_status: vec![TransactionInfoAndStatus {
                original_message_id: Some(original_msg_id.to_string()),
                original_instruction_id: original_instruction_id.map(String::from),
                original_end_to_end_id: None,
                transaction_status: status,
                status_reason_info: status_reason.map(|r| StatusReasonInfo {
                    reason: r,
                    additional_info: None,
                }),
            }],
        }
    }

    /// Parse from XML bytes
    pub fn parse_xml(_xml: &[u8]) -> Result<Self> {
        Err(Error::UnsupportedMessageType {
            msg_type: "XML parsing not yet implemented".into(),
        })
    }

    /// Generate XML bytes
    pub fn to_xml(&self) -> Result<Vec<u8>> {
        Err(Error::UnsupportedMessageType {
            msg_type: "XML generation not yet implemented".into(),
        })
    }

    /// Convert to ESF (eStream Format)
    pub fn to_esf(&self) -> Result<Vec<u8>> {
        use crate::esf::EsfBuilder;
        
        let mut builder = EsfBuilder::new(crate::MessageType::Pacs002);
        
        // Group header fields
        builder.add_string(0x1001, &self.group_header.message_id)?;
        builder.add_datetime(0x1002, &self.group_header.creation_date_time)?;
        
        // Transaction status fields (first transaction)
        if let Some(tx) = self.transaction_info_and_status.first() {
            if let Some(ref orig_id) = tx.original_instruction_id {
                builder.add_string(0x1101, orig_id)?;
            }
            builder.add_enum(0x1102, tx.transaction_status.code())?;
            
            if let Some(ref reason_info) = tx.status_reason_info {
                builder.add_enum(0x1103, reason_info.reason.code())?;
            }
        }
        
        builder.build()
    }
}

/// Group header for pacs.002
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StatusReportGroupHeader {
    /// Message identification
    pub message_id: String,
    /// Creation date and time
    pub creation_date_time: DateTime<Utc>,
}

/// Transaction information and status
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransactionInfoAndStatus {
    /// Original message identification
    pub original_message_id: Option<String>,
    /// Original instruction identification
    pub original_instruction_id: Option<String>,
    /// Original end-to-end identification
    pub original_end_to_end_id: Option<String>,
    /// Transaction status
    pub transaction_status: TransactionStatus,
    /// Status reason information
    pub status_reason_info: Option<StatusReasonInfo>,
}

/// Status reason information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StatusReasonInfo {
    /// Status reason
    pub reason: StatusReason,
    /// Additional information
    pub additional_info: Option<String>,
}

/// Status reason codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StatusReason {
    /// Account closed
    AccountClosed,
    /// Account blocked
    AccountBlocked,
    /// Insufficient funds
    InsufficientFunds,
    /// Invalid account
    InvalidAccount,
    /// Invalid debtor account
    InvalidDebtorAccount,
    /// Invalid creditor account
    InvalidCreditorAccount,
    /// No mandate
    NoMandate,
    /// Regulatory reason
    RegulatoryReason,
    /// Specific service offered by debtor agent
    AgentDecision,
    /// Other
    Other,
}

impl StatusReason {
    /// Get the ISO 20022 code
    pub fn code(&self) -> &'static str {
        match self {
            Self::AccountClosed => "AC04",
            Self::AccountBlocked => "AC06",
            Self::InsufficientFunds => "AM04",
            Self::InvalidAccount => "AC01",
            Self::InvalidDebtorAccount => "AC02",
            Self::InvalidCreditorAccount => "AC03",
            Self::NoMandate => "MD01",
            Self::RegulatoryReason => "RR04",
            Self::AgentDecision => "AGNT",
            Self::Other => "MS03",
        }
    }

    /// Parse from ISO 20022 code
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "AC04" => Some(Self::AccountClosed),
            "AC06" => Some(Self::AccountBlocked),
            "AM04" => Some(Self::InsufficientFunds),
            "AC01" => Some(Self::InvalidAccount),
            "AC02" => Some(Self::InvalidDebtorAccount),
            "AC03" => Some(Self::InvalidCreditorAccount),
            "MD01" => Some(Self::NoMandate),
            "RR04" => Some(Self::RegulatoryReason),
            "AGNT" => Some(Self::AgentDecision),
            "MS03" => Some(Self::Other),
            _ => None,
        }
    }
}
