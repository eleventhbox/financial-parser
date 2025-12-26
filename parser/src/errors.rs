use thiserror::Error;
use crate::model::transaction_type::TransactionType;

/// Errors raised when parsing transactions from different formats
///
/// This enum describes all possible errors which can be raised when:
/// - reading file (I/O),
/// - parsing CSV,
/// - validating data,
/// - deserializing binary data.
#[derive(Error, Debug)]
pub enum ParseError {
    /// Input output error (e.g, file not found or not readable).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// CSV file parsing error.
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    /// Transaction type parsing error.
    #[error("Transaction type parsing error: {0}")]
    InvalidTransactionType(String),
    /// Transaction status parsing error.
    #[error("Transaction status parsing error: {0}")]
    InvalidTransactionStatus(String),
    /// Transaction data validation error
    #[error("Validation error: {0}")]
    Validation(String),
    /// Error when CSV file header required fields do not match the actual fields.
    ///
    /// Contains:
    /// - `Vec<String>` — actual fields
    /// - `Vec<String>` — required fields
    #[error("Invalid headers: actual fields: {0:?}; required fields: {1:?}")]
    InvalidHeader(Vec<String>, Vec<String>),
    /// Error when CSV file record required fields count does not match the actual fields count.
    ///
    /// Contains:
    /// - `Vec<String>` — actual fields
    /// - `Vec<String>` — required fields
    #[error("Invalid record: actual fields: {0:?}; required fields: {1:?}")]
    InvalidRecord(Vec<String>, Vec<String>),
    /// actual "magic" value does not match the required value.
    ///
    /// Contains:
    /// - `[u8; 4]` — actual "magic" value
    /// - `[u8; 4]` — required "magic" value
    #[error("Invalid magic: {0:?}, expected: {1:?}")]
    InvalidMagic([u8; 4], [u8; 4]),
    /// Invalid amount for a transaction type.
    ///
    /// Contains:
    /// - `i64` — amount
    /// - `TransactionType` — transaction type
    #[error("Invalid amount: {0} for a transaction type {1:?}")]
    InvalidAmountForTransactionType(i64, TransactionType),
    /// Invalid transaction amount error.
    #[error("Invalid amount: {0}")]
    InvalidAmount(i64),
    /// Invalid description error.
    #[error("Invalid description: {0:?}")]
    InvalidDescription(String),
    /// Invaild data error.
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

