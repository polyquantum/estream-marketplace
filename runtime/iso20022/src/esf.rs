//! ESF (eStream Format) conversion utilities.

use alloc::vec::Vec;
use chrono::{DateTime, Utc};

use crate::types::{Amount, Bic, Iban};
use crate::{EsfFieldType, MessageType, PrivacyTier, Result, Error};

/// ESF Magic number
const ESF_MAGIC: u32 = 0x45534600; // "ESF\0"
/// ESF Version
const ESF_VERSION: u16 = 0x0100;

/// ESF message builder
pub struct EsfBuilder {
    msg_type: MessageType,
    fields: Vec<EsfField>,
}

/// Single ESF field
struct EsfField {
    id: u16,
    field_type: EsfFieldType,
    privacy: PrivacyTier,
    data: Vec<u8>,
}

impl EsfBuilder {
    /// Create a new ESF builder for the given message type
    pub fn new(msg_type: MessageType) -> Self {
        Self {
            msg_type,
            fields: Vec::new(),
        }
    }

    /// Add a string field
    pub fn add_string(&mut self, field_id: u16, value: &str) -> Result<()> {
        self.fields.push(EsfField {
            id: field_id,
            field_type: EsfFieldType::String,
            privacy: Self::get_privacy(field_id),
            data: value.as_bytes().to_vec(),
        });
        Ok(())
    }

    /// Add a datetime field
    pub fn add_datetime(&mut self, field_id: u16, value: &DateTime<Utc>) -> Result<()> {
        let timestamp = value.timestamp() as u64;
        self.fields.push(EsfField {
            id: field_id,
            field_type: EsfFieldType::DateTime,
            privacy: Self::get_privacy(field_id),
            data: timestamp.to_le_bytes().to_vec(),
        });
        Ok(())
    }

    /// Add a u32 field
    pub fn add_u32(&mut self, field_id: u16, value: u32) -> Result<()> {
        self.fields.push(EsfField {
            id: field_id,
            field_type: EsfFieldType::U32,
            privacy: Self::get_privacy(field_id),
            data: value.to_le_bytes().to_vec(),
        });
        Ok(())
    }

    /// Add an amount field (u128 cents)
    pub fn add_amount(&mut self, field_id: u16, amount: &Amount) -> Result<()> {
        let cents = amount.to_cents();
        self.fields.push(EsfField {
            id: field_id,
            field_type: EsfFieldType::U128,
            privacy: Self::get_privacy(field_id),
            data: cents.to_le_bytes().to_vec(),
        });
        Ok(())
    }

    /// Add an enum field (string code)
    pub fn add_enum(&mut self, field_id: u16, code: &str) -> Result<()> {
        self.fields.push(EsfField {
            id: field_id,
            field_type: EsfFieldType::Enum,
            privacy: Self::get_privacy(field_id),
            data: code.as_bytes().to_vec(),
        });
        Ok(())
    }

    /// Add a BIC field
    pub fn add_bic(&mut self, field_id: u16, bic: &Bic) -> Result<()> {
        self.fields.push(EsfField {
            id: field_id,
            field_type: EsfFieldType::Bic,
            privacy: Self::get_privacy(field_id),
            data: bic.0.as_bytes().to_vec(),
        });
        Ok(())
    }

    /// Add an IBAN field
    pub fn add_iban(&mut self, field_id: u16, iban: &Iban) -> Result<()> {
        self.fields.push(EsfField {
            id: field_id,
            field_type: EsfFieldType::Iban,
            privacy: Self::get_privacy(field_id),
            data: iban.0.as_bytes().to_vec(),
        });
        Ok(())
    }

    /// Get privacy tier for a field ID
    fn get_privacy(field_id: u16) -> PrivacyTier {
        // Privacy tiers based on field ID ranges (matches schema_rom.v)
        match field_id {
            0x0001..=0x00FF => PrivacyTier::Public,      // Header fields
            0x0100..=0x01FF => PrivacyTier::Restricted,  // Transaction IDs
            0x0200..=0x02FF => PrivacyTier::Restricted,  // Amounts
            0x0300..=0x03FF => PrivacyTier::Private,     // Party names
            0x0400..=0x04FF => PrivacyTier::Private,     // Account info
            0x0500..=0x05FF => PrivacyTier::Encrypted,   // Remittance info
            _ => PrivacyTier::Public,
        }
    }

    /// Build the final ESF message
    pub fn build(self) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(1024);

        // Header (16 bytes)
        output.extend_from_slice(&ESF_MAGIC.to_le_bytes());
        output.extend_from_slice(&ESF_VERSION.to_le_bytes());
        output.extend_from_slice(&(self.msg_type as u16).to_le_bytes());
        output.extend_from_slice(&(self.fields.len() as u16).to_le_bytes());
        
        // Placeholder for total length (will update later)
        let len_offset = output.len();
        output.extend_from_slice(&0u32.to_le_bytes());
        output.extend_from_slice(&[0u8; 2]); // Reserved

        // Fields
        for field in &self.fields {
            // Field header: ID (2) + Type (1) + Length (1) = 4 bytes
            output.extend_from_slice(&field.id.to_le_bytes());
            output.push(field.field_type as u8);
            output.push(field.data.len() as u8);
            output.extend_from_slice(&field.data);
        }

        // Update total length
        let total_len = output.len() as u32;
        output[len_offset..len_offset + 4].copy_from_slice(&total_len.to_le_bytes());

        // TODO: Add trailer with hash

        Ok(output)
    }
}

/// ESF message reader
pub struct EsfReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> EsfReader<'a> {
    /// Create a reader from ESF bytes
    pub fn new(data: &'a [u8]) -> Result<Self> {
        if data.len() < 16 {
            return Err(Error::EsfConversion {
                message: "ESF message too short".into(),
            });
        }

        // Validate magic
        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != ESF_MAGIC {
            return Err(Error::EsfConversion {
                message: "Invalid ESF magic number".into(),
            });
        }

        Ok(Self {
            data,
            offset: 16, // Skip header
        })
    }

    /// Get the message type
    pub fn message_type(&self) -> Option<MessageType> {
        let code = u16::from_le_bytes([self.data[6], self.data[7]]);
        MessageType::from_code(code)
    }

    /// Get the field count
    pub fn field_count(&self) -> u16 {
        u16::from_le_bytes([self.data[8], self.data[9]])
    }

    /// Read the next field
    pub fn next_field(&mut self) -> Option<(u16, EsfFieldType, &'a [u8])> {
        if self.offset + 4 > self.data.len() {
            return None;
        }

        let field_id = u16::from_le_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        let field_type = self.data[self.offset + 2];
        let field_len = self.data[self.offset + 3] as usize;

        if self.offset + 4 + field_len > self.data.len() {
            return None;
        }

        let field_data = &self.data[self.offset + 4..self.offset + 4 + field_len];
        self.offset += 4 + field_len;

        // Convert field type
        let ft = match field_type {
            0x01 => EsfFieldType::String,
            0x02 => EsfFieldType::U8,
            0x03 => EsfFieldType::U16,
            0x04 => EsfFieldType::U32,
            0x05 => EsfFieldType::U64,
            0x06 => EsfFieldType::U128,
            0x08 => EsfFieldType::Bytes,
            0x09 => EsfFieldType::Date,
            0x0A => EsfFieldType::DateTime,
            0x0B => EsfFieldType::Decimal,
            0x0C => EsfFieldType::Bic,
            0x0D => EsfFieldType::Iban,
            0x0E => EsfFieldType::Currency,
            0x0F => EsfFieldType::Enum,
            _ => EsfFieldType::None,
        };

        Some((field_id, ft, field_data))
    }
}
