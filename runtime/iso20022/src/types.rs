//! Common ISO 20022 types used across messages.

use alloc::string::String;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// ISO 20022 Amount with currency
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Amount {
    /// Amount value (supports up to 18 decimal places)
    pub value: Decimal,
    /// ISO 4217 currency code (3 characters)
    pub currency: String,
}

impl Amount {
    /// Create a new amount
    pub fn new(value: Decimal, currency: impl Into<String>) -> Self {
        Self {
            value,
            currency: currency.into(),
        }
    }

    /// Convert to cents (u128) for FPGA processing
    pub fn to_cents(&self) -> u128 {
        let scaled = self.value * Decimal::from(100);
        scaled.try_into().unwrap_or(0)
    }

    /// Create from cents (u128)
    pub fn from_cents(cents: u128, currency: impl Into<String>) -> Self {
        let value = Decimal::from(cents) / Decimal::from(100);
        Self {
            value,
            currency: currency.into(),
        }
    }
}

/// BIC (Bank Identifier Code) - 8 or 11 characters
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bic(pub String);

impl Bic {
    /// Create a new BIC, validating format
    pub fn new(bic: impl Into<String>) -> crate::Result<Self> {
        let bic = bic.into();
        if bic.len() != 8 && bic.len() != 11 {
            return Err(crate::Error::InvalidBic { value: bic });
        }
        // Basic format validation
        if !bic.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(crate::Error::InvalidBic { value: bic });
        }
        Ok(Self(bic))
    }

    /// Get the institution code (first 4 characters)
    pub fn institution_code(&self) -> &str {
        &self.0[0..4]
    }

    /// Get the country code (characters 5-6)
    pub fn country_code(&self) -> &str {
        &self.0[4..6]
    }

    /// Get the location code (characters 7-8)
    pub fn location_code(&self) -> &str {
        &self.0[6..8]
    }

    /// Get the branch code (characters 9-11, if present)
    pub fn branch_code(&self) -> Option<&str> {
        if self.0.len() == 11 {
            Some(&self.0[8..11])
        } else {
            None
        }
    }
}

/// IBAN (International Bank Account Number)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Iban(pub String);

impl Iban {
    /// Create a new IBAN, validating format
    pub fn new(iban: impl Into<String>) -> crate::Result<Self> {
        let iban = iban.into().replace(' ', "").to_uppercase();
        if iban.len() < 15 || iban.len() > 34 {
            return Err(crate::Error::InvalidIban { value: iban });
        }
        // TODO: Add checksum validation
        Ok(Self(iban))
    }

    /// Get the country code (first 2 characters)
    pub fn country_code(&self) -> &str {
        &self.0[0..2]
    }

    /// Get the check digits (characters 3-4)
    pub fn check_digits(&self) -> &str {
        &self.0[2..4]
    }

    /// Get the BBAN (Basic Bank Account Number)
    pub fn bban(&self) -> &str {
        &self.0[4..]
    }
}

/// Party identification
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PartyIdentification {
    /// Party name
    pub name: Option<String>,
    /// Postal address
    pub postal_address: Option<PostalAddress>,
    /// Contact details
    pub contact_details: Option<ContactDetails>,
}

/// Postal address
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PostalAddress {
    /// Street name
    pub street_name: Option<String>,
    /// Building number
    pub building_number: Option<String>,
    /// Post code
    pub post_code: Option<String>,
    /// Town name
    pub town_name: Option<String>,
    /// Country code (ISO 3166-1 alpha-2)
    pub country: Option<String>,
}

/// Contact details
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContactDetails {
    /// Phone number
    pub phone_number: Option<String>,
    /// Email address
    pub email_address: Option<String>,
}

/// Account identification
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AccountId {
    /// IBAN
    Iban(Iban),
    /// Other identification
    Other(String),
}

/// Financial institution identification
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FinancialInstitutionId {
    /// BIC
    pub bic: Option<Bic>,
    /// Clearing system member identification
    pub clearing_system_member_id: Option<String>,
    /// Name
    pub name: Option<String>,
}

/// Payment identification
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PaymentId {
    /// Instruction identification (unique ID assigned by instructing party)
    pub instruction_id: Option<String>,
    /// End-to-end identification (unique ID from debtor to creditor)
    pub end_to_end_id: String,
    /// Transaction identification (unique ID assigned by first agent)
    pub transaction_id: Option<String>,
    /// UETR (Unique End-to-end Transaction Reference)
    pub uetr: Option<String>,
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum TransactionStatus {
    /// Accepted Settlement Completed
    AcceptedSettlementCompleted = 0x01,
    /// Accepted Settlement In Progress
    AcceptedSettlementInProgress = 0x02,
    /// Pending
    Pending = 0x03,
    /// Rejected
    Rejected = 0x04,
    /// Accepted Technical Validation
    AcceptedTechnicalValidation = 0x05,
    /// Accepted Customer Profile
    AcceptedCustomerProfile = 0x06,
}

impl TransactionStatus {
    /// Get the ISO 20022 code
    pub fn code(&self) -> &'static str {
        match self {
            Self::AcceptedSettlementCompleted => "ACSC",
            Self::AcceptedSettlementInProgress => "ACSP",
            Self::Pending => "PDNG",
            Self::Rejected => "RJCT",
            Self::AcceptedTechnicalValidation => "ACTC",
            Self::AcceptedCustomerProfile => "ACCP",
        }
    }

    /// Parse from ISO 20022 code
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "ACSC" => Some(Self::AcceptedSettlementCompleted),
            "ACSP" => Some(Self::AcceptedSettlementInProgress),
            "PDNG" => Some(Self::Pending),
            "RJCT" => Some(Self::Rejected),
            "ACTC" => Some(Self::AcceptedTechnicalValidation),
            "ACCP" => Some(Self::AcceptedCustomerProfile),
            _ => None,
        }
    }
}

/// Charge bearer type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ChargeBearer {
    /// Borne by debtor
    Debt = 0x01,
    /// Borne by creditor
    Cred = 0x02,
    /// Shared between debtor and creditor
    Shar = 0x03,
    /// Following service level
    Slev = 0x04,
}

impl ChargeBearer {
    /// Get the ISO 20022 code
    pub fn code(&self) -> &'static str {
        match self {
            Self::Debt => "DEBT",
            Self::Cred => "CRED",
            Self::Shar => "SHAR",
            Self::Slev => "SLEV",
        }
    }

    /// Parse from ISO 20022 code
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "DEBT" => Some(Self::Debt),
            "CRED" => Some(Self::Cred),
            "SHAR" => Some(Self::Shar),
            "SLEV" => Some(Self::Slev),
            _ => None,
        }
    }
}

/// Remittance information
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RemittanceInfo {
    /// Unstructured remittance info (free text)
    pub unstructured: Option<String>,
    /// Structured remittance info
    pub structured: Option<StructuredRemittanceInfo>,
}

/// Structured remittance information
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StructuredRemittanceInfo {
    /// Creditor reference information
    pub creditor_reference: Option<String>,
    /// Referred document information
    pub referred_document: Option<ReferredDocument>,
}

/// Referred document information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReferredDocument {
    /// Document type
    pub document_type: String,
    /// Document number
    pub number: String,
    /// Document date
    pub date: Option<NaiveDate>,
}
