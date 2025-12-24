use serde::Serialize;
use crate::errors::ParseError;

#[derive(Debug, PartialEq, Serialize)]
pub enum TransactionStatus {
    Success,
    Failure,
    Pending
}

impl TransactionStatus {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Failure => 1,
            Self::Pending => 2,
        }
    }

    pub fn from_u8(value: u8) -> Result<Self, ParseError> {
        match value {
            0 => Ok(Self::Success),
            1 => Ok(Self::Failure),
            2 => Ok(Self::Pending),
            _ => Err(ParseError::InvalidTransactionStatus(format!("Invalid transaction status: {}", value))),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        match s.to_uppercase().as_str() {
            "SUCCESS" => Ok(Self::Success),
            "FAILURE" => Ok(Self::Failure),
            "PENDING" => Ok(Self::Pending),
            _ => Err(ParseError::InvalidTransactionStatus(format!("Invalid transaction status string: {}", s))),
        }
    }
}

impl std::str::FromStr for TransactionStatus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TransactionStatus::from_str(s)
    }
}