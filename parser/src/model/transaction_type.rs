use serde::Serialize;
use crate::errors::ParseError;

#[derive(Debug, PartialEq, Serialize)]
pub enum TransactionType {
    Deposit,
    Transfer,
    Withdrawal
}

impl TransactionType {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Deposit => 0,
            Self::Transfer => 1,
            Self::Withdrawal => 2,
        }
    }

    pub fn from_u8(value: u8) -> Result<Self, ParseError> {
        match value {
            0 => Ok(Self::Deposit),
            1 => Ok(Self::Transfer),
            2 => Ok(Self::Withdrawal),
            _ => Err(ParseError::InvalidTransactionType(format!("Invalid transaction type: {}", value))),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        match s.to_uppercase().as_str() {
            "DEPOSIT" => Ok(Self::Deposit),
            "TRANSFER" => Ok(Self::Transfer),
            "WITHDRAWAL" => Ok(Self::Withdrawal),
            _ => Err(ParseError::InvalidTransactionType(format!("Invalid transaction type string: {}", s))),
        }
    }
}

impl std::str::FromStr for TransactionType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TransactionType::from_str(s)
    }
}