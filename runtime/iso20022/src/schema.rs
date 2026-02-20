//! Schema definitions for ISO 20022 field mappings.
//!
//! This module defines the field ID mappings that match the FPGA schema_rom.v.

use crate::{EsfFieldType, PrivacyTier};

/// Schema field definition
#[derive(Debug, Clone)]
pub struct FieldDefinition {
    /// XPath in ISO 20022 message
    pub xpath: &'static str,
    /// ESF field ID
    pub field_id: u16,
    /// ESF field type
    pub field_type: EsfFieldType,
    /// Privacy tier
    pub privacy_tier: PrivacyTier,
    /// Whether the field is required
    pub required: bool,
    /// Maximum length (for strings)
    pub max_length: Option<usize>,
}

/// Field definitions for pacs.008
pub const PACS008_FIELDS: &[FieldDefinition] = &[
    // Group Header
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/GrpHdr/MsgId",
        field_id: 0x0001,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Public,
        required: true,
        max_length: Some(35),
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/GrpHdr/CreDtTm",
        field_id: 0x0002,
        field_type: EsfFieldType::DateTime,
        privacy_tier: PrivacyTier::Public,
        required: true,
        max_length: None,
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/GrpHdr/NbOfTxs",
        field_id: 0x0003,
        field_type: EsfFieldType::U32,
        privacy_tier: PrivacyTier::Public,
        required: true,
        max_length: None,
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/GrpHdr/SttlmInf/SttlmMtd",
        field_id: 0x0004,
        field_type: EsfFieldType::Enum,
        privacy_tier: PrivacyTier::Public,
        required: true,
        max_length: Some(4),
    },
    
    // Transaction - Payment ID
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/InstrId",
        field_id: 0x0101,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Restricted,
        required: false,
        max_length: Some(35),
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/EndToEndId",
        field_id: 0x0102,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Restricted,
        required: true,
        max_length: Some(35),
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/TxId",
        field_id: 0x0103,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Restricted,
        required: false,
        max_length: Some(35),
    },
    
    // Amount
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt",
        field_id: 0x0201,
        field_type: EsfFieldType::U128,
        privacy_tier: PrivacyTier::Restricted,
        required: true,
        max_length: None,
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt/@Ccy",
        field_id: 0x0202,
        field_type: EsfFieldType::Currency,
        privacy_tier: PrivacyTier::Restricted,
        required: true,
        max_length: Some(3),
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr",
        field_id: 0x0203,
        field_type: EsfFieldType::Enum,
        privacy_tier: PrivacyTier::Public,
        required: false,
        max_length: Some(4),
    },
    
    // Debtor
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Dbtr/Nm",
        field_id: 0x0301,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Private,
        required: false,
        max_length: Some(140),
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Dbtr/PstlAdr/Ctry",
        field_id: 0x0302,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Private,
        required: false,
        max_length: Some(2),
    },
    
    // Creditor
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Cdtr/Nm",
        field_id: 0x0303,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Private,
        required: false,
        max_length: Some(140),
    },
    
    // Accounts
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/DbtrAcct/Id/IBAN",
        field_id: 0x0401,
        field_type: EsfFieldType::Iban,
        privacy_tier: PrivacyTier::Private,
        required: false,
        max_length: Some(34),
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/DbtrAgt/FinInstnId/BICFI",
        field_id: 0x0402,
        field_type: EsfFieldType::Bic,
        privacy_tier: PrivacyTier::Restricted,
        required: false,
        max_length: Some(11),
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/CdtrAcct/Id/IBAN",
        field_id: 0x0403,
        field_type: EsfFieldType::Iban,
        privacy_tier: PrivacyTier::Private,
        required: false,
        max_length: Some(34),
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/CdtrAgt/FinInstnId/BICFI",
        field_id: 0x0404,
        field_type: EsfFieldType::Bic,
        privacy_tier: PrivacyTier::Restricted,
        required: false,
        max_length: Some(11),
    },
    
    // Remittance
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/RmtInf/Ustrd",
        field_id: 0x0501,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Encrypted,
        required: false,
        max_length: Some(140),
    },
    FieldDefinition {
        xpath: "/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/RmtInf/Strd/CdtrRefInf/Ref",
        field_id: 0x0502,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Private,
        required: false,
        max_length: Some(35),
    },
];

/// Field definitions for pacs.002
pub const PACS002_FIELDS: &[FieldDefinition] = &[
    FieldDefinition {
        xpath: "/Document/FIToFIPmtStsRpt/GrpHdr/MsgId",
        field_id: 0x1001,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Public,
        required: true,
        max_length: Some(35),
    },
    FieldDefinition {
        xpath: "/Document/FIToFIPmtStsRpt/GrpHdr/CreDtTm",
        field_id: 0x1002,
        field_type: EsfFieldType::DateTime,
        privacy_tier: PrivacyTier::Public,
        required: true,
        max_length: None,
    },
    FieldDefinition {
        xpath: "/Document/FIToFIPmtStsRpt/TxInfAndSts/OrgnlInstrId",
        field_id: 0x1101,
        field_type: EsfFieldType::String,
        privacy_tier: PrivacyTier::Restricted,
        required: false,
        max_length: Some(35),
    },
    FieldDefinition {
        xpath: "/Document/FIToFIPmtStsRpt/TxInfAndSts/TxSts",
        field_id: 0x1102,
        field_type: EsfFieldType::Enum,
        privacy_tier: PrivacyTier::Public,
        required: true,
        max_length: Some(4),
    },
    FieldDefinition {
        xpath: "/Document/FIToFIPmtStsRpt/TxInfAndSts/StsRsnInf/Rsn/Cd",
        field_id: 0x1103,
        field_type: EsfFieldType::Enum,
        privacy_tier: PrivacyTier::Public,
        required: false,
        max_length: Some(4),
    },
];

/// Compute FNV-1a hash of an XPath (matches FPGA tree_walker_fsm.v)
pub fn compute_path_hash(xpath: &str) -> u32 {
    const FNV_OFFSET_BASIS: u32 = 0x811c9dc5;
    const FNV_PRIME: u32 = 0x01000193;

    let mut hash = FNV_OFFSET_BASIS;
    for byte in xpath.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_hash_consistency() {
        // These hashes should match the values in schema_rom.v
        let hash = compute_path_hash("/Document/FIToFICstmrCdtTrf/GrpHdr/MsgId");
        assert!(hash != 0);
        
        // Different paths should produce different hashes
        let hash2 = compute_path_hash("/Document/FIToFICstmrCdtTrf/GrpHdr/CreDtTm");
        assert_ne!(hash, hash2);
    }
}
