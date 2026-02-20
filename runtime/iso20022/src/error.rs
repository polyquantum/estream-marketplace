//! Error types for ISO 20022 parsing and conversion.

use thiserror::Error;

/// Result type alias for ISO 20022 operations
pub type Result<T> = core::result::Result<T, Error>;

/// ISO 20022 parsing and conversion errors
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid XML syntax
    #[error("Invalid XML syntax at offset {offset}: {message}")]
    XmlSyntax { offset: usize, message: String },

    /// Unknown element in XML
    #[error("Unknown element '{element}' at path '{path}'")]
    UnknownElement { element: String, path: String },

    /// Schema validation failed
    #[error("Schema validation failed: {message}")]
    SchemaValidation { message: String },

    /// Required field missing
    #[error("Required field '{field}' is missing")]
    MissingField { field: String },

    /// Invalid field value
    #[error("Invalid value for field '{field}': {message}")]
    InvalidFieldValue { field: String, message: String },

    /// Field overflow (value too long)
    #[error("Field '{field}' exceeds maximum length of {max_len}")]
    FieldOverflow { field: String, max_len: usize },

    /// Depth overflow (too deeply nested)
    #[error("Document depth exceeds maximum of {max_depth}")]
    DepthOverflow { max_depth: usize },

    /// Tag mismatch (close tag doesn't match open tag)
    #[error("Tag mismatch: expected '</{expected}>', found '</{found}>'")]
    TagMismatch { expected: String, found: String },

    /// Invalid character encoding
    #[error("Invalid character encoding at offset {offset}")]
    InvalidEncoding { offset: usize },

    /// Invalid amount format
    #[error("Invalid amount format: {value}")]
    InvalidAmount { value: String },

    /// Invalid date format
    #[error("Invalid date format: {value}")]
    InvalidDate { value: String },

    /// Invalid BIC format
    #[error("Invalid BIC format: {value}")]
    InvalidBic { value: String },

    /// Invalid IBAN format
    #[error("Invalid IBAN format: {value}")]
    InvalidIban { value: String },

    /// Invalid currency code
    #[error("Invalid currency code: {value}")]
    InvalidCurrency { value: String },

    /// Unsupported message type
    #[error("Unsupported message type: {msg_type}")]
    UnsupportedMessageType { msg_type: String },

    /// ESF conversion error
    #[error("ESF conversion error: {message}")]
    EsfConversion { message: String },

    /// FPGA communication error
    #[cfg(feature = "fpga")]
    #[error("FPGA error: {message}")]
    FpgaError { message: String },
}

/// Error codes matching FPGA parser (E3xxx series)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ErrorCode {
    /// E3001: Invalid XML syntax
    Syntax = 0x3001,
    /// E3002: Unknown element
    UnknownElement = 0x3002,
    /// E3003: Schema validation failed
    Schema = 0x3003,
    /// E3004: Field overflow
    Overflow = 0x3004,
    /// E3005: Depth overflow
    Depth = 0x3005,
    /// E3006: Tag mismatch
    Mismatch = 0x3006,
    /// E3007: Invalid encoding
    Encoding = 0x3007,
    /// E3008: Timeout
    Timeout = 0x3008,
}

impl From<&Error> for ErrorCode {
    fn from(err: &Error) -> Self {
        match err {
            Error::XmlSyntax { .. } => ErrorCode::Syntax,
            Error::UnknownElement { .. } => ErrorCode::UnknownElement,
            Error::SchemaValidation { .. } => ErrorCode::Schema,
            Error::FieldOverflow { .. } => ErrorCode::Overflow,
            Error::DepthOverflow { .. } => ErrorCode::Depth,
            Error::TagMismatch { .. } => ErrorCode::Mismatch,
            Error::InvalidEncoding { .. } => ErrorCode::Encoding,
            _ => ErrorCode::Schema,
        }
    }
}
