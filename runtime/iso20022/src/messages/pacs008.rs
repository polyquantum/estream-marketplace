//! pacs.008 - FIToFICustomerCreditTransfer message.

use alloc::string::String;
use alloc::vec::Vec;
use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::*;
use crate::{Error, Result};

/// pacs.008 - FI to FI Customer Credit Transfer
///
/// This message is sent by the debtor agent to the creditor agent, directly or
/// through other agents and/or a payment clearing and settlement system.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pacs008 {
    /// Group header
    pub group_header: GroupHeader,
    /// Credit transfer transactions
    pub credit_transfer_transactions: Vec<CreditTransferTransaction>,
}

impl Pacs008 {
    /// Message identifier
    pub const MESSAGE_ID: &'static str = "pacs.008.001.08";

    /// Parse from XML bytes
    pub fn parse_xml(_xml: &[u8]) -> Result<Self> {
        // TODO: Implement XML parsing (would use quick-xml or similar)
        Err(Error::UnsupportedMessageType {
            msg_type: "XML parsing not yet implemented".into(),
        })
    }

    /// Generate XML bytes
    pub fn to_xml(&self) -> Result<Vec<u8>> {
        // TODO: Implement XML generation
        Err(Error::UnsupportedMessageType {
            msg_type: "XML generation not yet implemented".into(),
        })
    }

    /// Convert to ESF (eStream Format)
    pub fn to_esf(&self) -> Result<Vec<u8>> {
        use crate::esf::EsfBuilder;
        
        let mut builder = EsfBuilder::new(crate::MessageType::Pacs008);
        
        // Group header fields
        builder.add_string(0x0001, &self.group_header.message_id)?;
        builder.add_datetime(0x0002, &self.group_header.creation_date_time)?;
        builder.add_u32(0x0003, self.group_header.number_of_transactions)?;
        
        // Transaction fields (first transaction for now)
        if let Some(tx) = self.credit_transfer_transactions.first() {
            if let Some(ref instr_id) = tx.payment_id.instruction_id {
                builder.add_string(0x0101, instr_id)?;
            }
            builder.add_string(0x0102, &tx.payment_id.end_to_end_id)?;
            
            // Amount
            builder.add_amount(0x0201, &tx.interbank_settlement_amount)?;
            builder.add_string(0x0202, &tx.interbank_settlement_amount.currency)?;
            
            // Charge bearer
            if let Some(cb) = tx.charge_bearer {
                builder.add_enum(0x0203, cb.code())?;
            }
            
            // Debtor
            if let Some(ref name) = tx.debtor.name {
                builder.add_string(0x0301, name)?;
            }
            
            // Debtor account
            if let Some(ref acct) = tx.debtor_account {
                match acct {
                    AccountId::Iban(iban) => builder.add_iban(0x0401, iban)?,
                    AccountId::Other(id) => builder.add_string(0x0401, id)?,
                }
            }
            
            // Debtor agent
            if let Some(ref bic) = tx.debtor_agent.bic {
                builder.add_bic(0x0402, bic)?;
            }
            
            // Creditor
            if let Some(ref name) = tx.creditor.name {
                builder.add_string(0x0303, name)?;
            }
            
            // Creditor account
            if let Some(ref acct) = tx.creditor_account {
                match acct {
                    AccountId::Iban(iban) => builder.add_iban(0x0403, iban)?,
                    AccountId::Other(id) => builder.add_string(0x0403, id)?,
                }
            }
            
            // Creditor agent
            if let Some(ref bic) = tx.creditor_agent.bic {
                builder.add_bic(0x0404, bic)?;
            }
            
            // Remittance info
            if let Some(ref rmti) = tx.remittance_info {
                if let Some(ref ustrd) = rmti.unstructured {
                    builder.add_string(0x0501, ustrd)?;
                }
            }
        }
        
        builder.build()
    }
}

/// Group header for pacs.008
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GroupHeader {
    /// Message identification (unique per message)
    pub message_id: String,
    /// Creation date and time
    pub creation_date_time: DateTime<Utc>,
    /// Number of transactions
    pub number_of_transactions: u32,
    /// Total interbank settlement amount (optional)
    pub total_interbank_settlement_amount: Option<Amount>,
    /// Settlement information
    pub settlement_info: SettlementInfo,
}

/// Settlement information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SettlementInfo {
    /// Settlement method
    pub settlement_method: SettlementMethod,
}

/// Settlement method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SettlementMethod {
    /// Instructed agent
    InstructedAgent,
    /// Instructing agent
    InstructingAgent,
    /// Clearing system
    ClearingSystem,
}

impl SettlementMethod {
    /// Get the ISO 20022 code
    pub fn code(&self) -> &'static str {
        match self {
            Self::InstructedAgent => "INDA",
            Self::InstructingAgent => "INGA",
            Self::ClearingSystem => "CLRG",
        }
    }
}

/// Credit transfer transaction
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreditTransferTransaction {
    /// Payment identification
    pub payment_id: PaymentId,
    /// Interbank settlement amount
    pub interbank_settlement_amount: Amount,
    /// Interbank settlement date (optional)
    pub interbank_settlement_date: Option<chrono::NaiveDate>,
    /// Charge bearer
    pub charge_bearer: Option<ChargeBearer>,
    /// Debtor
    pub debtor: PartyIdentification,
    /// Debtor account
    pub debtor_account: Option<AccountId>,
    /// Debtor agent
    pub debtor_agent: FinancialInstitutionId,
    /// Creditor agent
    pub creditor_agent: FinancialInstitutionId,
    /// Creditor
    pub creditor: PartyIdentification,
    /// Creditor account
    pub creditor_account: Option<AccountId>,
    /// Remittance information
    pub remittance_info: Option<RemittanceInfo>,
}
