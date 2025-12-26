use serde::Serialize;
use crate::errors::ParseError;
use clap::ValueEnum;
use strum_macros::{EnumString, Display};

/// Acceptable transaction statuses
#[derive(Debug, PartialEq, Serialize)]
#[derive(Clone, Copy, Eq, ValueEnum)]
#[derive(EnumString, Display)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionStatus {
    /// Successful transaction
    Success,
    /// Failed transaction
    Failure,
    /// Pending transaction
    Pending
}

impl TransactionStatus {
    /// # Returning value
    ///
    /// Returns `u8` - transaction type u8 representation
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Failure => 1,
            Self::Pending => 2,
        }
    }

    /// # Returning value
    ///
    /// Returns `Result<Self, ParseError>`:
    /// - `Ok(TransactionType)` - successful parsing result
    /// - `Err(ParseError)` - parsing error
    pub fn from_u8(value: u8) -> Result<Self, ParseError> {
        match value {
            0 => Ok(Self::Success),
            1 => Ok(Self::Failure),
            2 => Ok(Self::Pending),
            _ => Err(ParseError::InvalidTransactionStatus(format!("Invalid transaction status: {}", value))),
        }
    }
    
}