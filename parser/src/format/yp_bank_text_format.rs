use crate::errors::ParseError;
use crate::format::common::{parse_description, parse_number, parse_transaction_status, parse_transaction_type};
use crate::model::transaction::Transaction;
use crate::model::transaction_status::TransactionStatus;
use crate::model::transaction_type::TransactionType;
use std::collections::HashMap;
use std::io::{BufRead, Read};

pub struct YPBankTextParser;
impl YPBankTextParser {
    /// Parses data in YPBankTextFormat from different sources and return transactions
    /// 
    /// Checks if data conforms to format requirements
    pub fn parse<R: Read>(reader: R) -> Result<Vec<Transaction>, ParseError> {
        let reader = std::io::BufReader::new(reader);
        let mut transactions = Vec::new();
        let mut current_record = HashMap::new();
        let mut line_number = 0;
        for line in reader.lines() {
            line_number += 1;
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                if !current_record.is_empty() {
                    let transaction = Self::parse_record(&current_record, line_number)?;
                    transactions.push(transaction);
                    current_record.clear();
                }
                continue;
            }
            if line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err(ParseError::Validation(format!(
                    "Invalid line format at line {}: '{}'",
                    line_number, line
                )));
            }
            let key = parts[0].trim();
            let value = parts[1].trim();
            if current_record.contains_key(key) {
                return Err(ParseError::Validation(format!(
                    "Duplicate key '{}' in transaction at line {}",
                    key, line_number
                )));
            }
            current_record.insert(key.to_string(), value.to_string());
        }
        if !current_record.is_empty() {
            let transaction = Self::parse_record(&current_record, line_number)?;
            transactions.push(transaction);
        }
        Ok(transactions)
    }

    /// Writes transactions to different targets in YPBankTextFormat 
    /// 
    /// Can add comments to output record
    pub fn write<W: std::io::Write>(
        transactions: &[Transaction],
        writer: &mut W,
        include_comments: bool,
    ) -> Result<(), ParseError> {
        for (i, transaction) in transactions.iter().enumerate() {
            if include_comments {
                writeln!(writer, "# Record {} ({}): {}",
                         i + 1,
                         match transaction.tx_type {
                             TransactionType::Deposit => "Deposit",
                             TransactionType::Transfer => "Transfer",
                             TransactionType::Withdrawal => "Withdrawal",
                         },
                         transaction.description
                )?;
            }
            writeln!(writer, "TX_ID: {}", transaction.tx_id)?;
            writeln!(writer, "TX_TYPE: {}", match transaction.tx_type {
                TransactionType::Deposit => "DEPOSIT",
                TransactionType::Transfer => "TRANSFER",
                TransactionType::Withdrawal => "WITHDRAWAL",
            })?;
            writeln!(writer, "FROM_USER_ID: {}", transaction.from_user_id)?;
            writeln!(writer, "TO_USER_ID: {}", transaction.to_user_id)?;
            if transaction.amount <= 0 {
                return Err(ParseError::InvalidAmount(transaction.amount))
            }
            writeln!(writer, "AMOUNT: {}", transaction.amount)?;
            writeln!(writer, "TIMESTAMP: {}", transaction.timestamp)?;
            writeln!(writer, "STATUS: {}", match transaction.status {
                TransactionStatus::Success => "SUCCESS",
                TransactionStatus::Failure => "FAILURE",
                TransactionStatus::Pending => "PENDING",
            })?;
            writeln!(writer, "DESCRIPTION: \"{}\"", transaction.description)?;
            if i < transactions.len() - 1 {
                writeln!(writer)?;
            }
        }
        Ok(())
    }

    /// Parses a single record to return a transaction
    fn parse_record(
        record: &HashMap<String, String>,
        line_number: usize,
    ) -> Result<Transaction, ParseError> {
        let required_fields = [
            "TX_ID", "TX_TYPE", "FROM_USER_ID", "TO_USER_ID",
            "AMOUNT", "TIMESTAMP", "STATUS", "DESCRIPTION"
        ];
        for &field in &required_fields {
            if !record.contains_key(field) {
                return Err(ParseError::Validation(format!(
                    "Missing required field '{}' in transaction ending at line {}",
                    field, line_number
                )));
            }
        }
        let tx_id = parse_number("TX_ID", record.get("TX_ID").expect("TX_ID is not empty"), line_number)?;
        let tx_type = parse_transaction_type(record.get("TX_TYPE").expect("TX_TYPE is not empty"), line_number)?;
        let from_user_id = parse_number("FROM_USER_ID", record.get("FROM_USER_ID").expect("FROM_USER_ID is not empty"), line_number)?;
        let to_user_id = parse_number("TO_USER_ID", record.get("TO_USER_ID").expect("TO_USER_ID is not empty"), line_number)?;
        let amount = parse_number("AMOUNT", record.get("AMOUNT").expect("AMOUNT is not empty"), line_number)?;
        if amount <= 0 {
            return Err(ParseError::InvalidAmount(amount))
        }
        let timestamp = parse_number("TIMESTAMP", record.get("TIMESTAMP").expect("TIMESTAMP is not empty"), line_number)?;
        let status = parse_transaction_status(record.get("STATUS").expect("STATUS is not empty"), line_number)?;
        let description = parse_description(record.get("DESCRIPTION").expect("DESCRIPTION is not empty"), line_number)?;
        let transaction = Transaction {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        };
        transaction.validate()?;
        Ok(transaction)
    }
}