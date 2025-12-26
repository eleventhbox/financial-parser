use std::fmt;
use serde::Serialize;
use crate::errors::ParseError;
use crate::model::transaction_type::TransactionType;
use crate::model::transaction_status::TransactionStatus;

/// Transaction
#[derive(Debug, Serialize, PartialEq)]
pub struct Transaction {
    /// Transaction identifier
    pub tx_id: u64,
    /// Transaction type
    pub tx_type: TransactionType,
    /// User id for transfer and withdrawal
    pub from_user_id: u64,
    /// User id for transfer and deposit
    pub to_user_id: u64,
    /// Transaction amount
    pub amount: i64,
    /// Transaction timestamp in Unix epoch millis
    pub timestamp: u64,
    /// Transaction status
    pub status: TransactionStatus,
    /// Transaction desription
    pub description: String
}

impl fmt::Display for Transaction {
    /// Transaction formatted representation
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction {{ tx_id: {}, tx_type: {:?}, amount: {}, timestamp: {}, description: {} }}",
            self.tx_id,
            self.tx_type,
            self.amount,
            self.formatted_timestamp(),
            self.description
        )
    }
}

impl Transaction {
    /// Unix epoch timestamp millis to String conversion
    ///
    /// # Returning value
    ///
    /// Returns `String` - timestamp string representation in format "%Y-%m-%d %H:%M:%S%.3f"
    pub fn formatted_timestamp(&self) -> String {
        let seconds = self.timestamp / 1000;
        let millis = self.timestamp % 1000;
        if let Some(dt) = chrono::DateTime::from_timestamp(seconds as i64, (millis * 1_000_000) as u32) {
            format!("{}", dt.format("%Y-%m-%d %H:%M:%S%.3f"))
        } else {
            format!("Invalid timestamp: {}", self.timestamp)
        }
    }
    
    /// Common transaction requirements validation
    ///
    /// # Returning value
    ///
    /// Returns `Result<(), ParseError>`:
    /// - Ok(()) - successful transaction validation
    /// - Err(ParseError) - parsing error
    pub fn validate(&self) -> Result<(), ParseError> {
        if self.tx_type == TransactionType::Deposit && self.from_user_id != 0 {
            return Err(ParseError::Validation(format!(
                "FROM_USER_ID must be 0 for DEPOSIT, got {}",
                self.from_user_id
            )));
        }
        if self.tx_type == TransactionType::Withdrawal && self.to_user_id != 0 {
            return Err(ParseError::Validation(format!(
                "TO_USER_ID must be 0 for WITHDRAWAL, got {}",
                self.to_user_id
            )));
        }
        Ok(())
    }
}