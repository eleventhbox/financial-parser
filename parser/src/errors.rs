use thiserror::Error;
use crate::model::transaction_type::TransactionType;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Transaction type parsing error: {0}")]
    InvalidTransactionType(String),
    #[error("Transaction status parsing error: {0}")]
    InvalidTransactionStatus(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Invalid headers: actual fields: {0:?}; required fields: {1:?}")]
    InvalidHeader(Vec<String>, Vec<String>),
    #[error("Invalid record: actual fields: {0:?}; required fields: {1:?}")]
    InvalidRecord(Vec<String>, Vec<String>),
    #[error("Invalid magic: {0:?}, expected: {1:?}")]
    InvalidMagic([u8; 4], [u8; 4]),
    #[error("Invalid amount: {0} for a transaction type {1:?}")]
    InvalidAmountForTransactionType(i64, TransactionType),
    #[error("Invalid amount: {0}")]
    InvalidAmount(i64),
    #[error("Invalid description: {0:?}")]
    InvalidDescription(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

