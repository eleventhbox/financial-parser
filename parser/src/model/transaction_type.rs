use serde::Serialize;
use crate::errors::ParseError;
use clap::ValueEnum;
use strum_macros::{EnumString, Display};

/// Acceptable transaction types
#[derive(Debug, PartialEq, Serialize)]
#[derive(Clone, Copy, Eq, ValueEnum)]
#[derive(EnumString, Display)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionType {
    /// Account replenishment 
    Deposit,
    /// Transfer between accounts
    Transfer,
    /// Account withdrawal
    Withdrawal
}

impl TransactionType {
    /// # Returning value
    ///
    /// Returns `u8` - transaction type u8 representation
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Deposit => 0,
            Self::Transfer => 1,
            Self::Withdrawal => 2,
        }
    }

    /// # Returning value
    ///
    /// Returns `Result<Self, ParseError>`:
    /// - `Ok(TransactionType)` - successful parsing result
    /// - `Err(ParseError)` - parsing error
    pub fn from_u8(value: u8) -> Result<Self, ParseError> {
        match value {
            0 => Ok(Self::Deposit),
            1 => Ok(Self::Transfer),
            2 => Ok(Self::Withdrawal),
            _ => Err(ParseError::InvalidTransactionType(format!("Invalid transaction type: {}", value))),
        }
    }
    
}