use crate::errors::ParseError;
use crate::format::common::{parse_number, parse_transaction_status, parse_transaction_type};
use crate::model::transaction::Transaction;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::io::Read;

const REQUIRED_FIELDS: &[&str] = &[
    "TX_ID", "TX_TYPE", "FROM_USER_ID", "TO_USER_ID",
    "AMOUNT", "TIMESTAMP", "STATUS", "DESCRIPTION"
];

pub struct YPBankCsvParser;
impl YPBankCsvParser {
    /// Parses data in YPBankCsvFormat from different sources and return transactions
    /// 
    /// Checks if data conforms to format requirements
    pub fn parse<R: Read>(reader: R) -> Result<Vec<Transaction>, ParseError> {
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .flexible(false)
            .quoting(true)
            .from_reader(reader);
        let mut transactions = Vec::new();
        let headers = csv_reader.headers()?;
        if let Err(e) = Self::validate_headers(headers) {
            Err(e)
        } else {
            for (line_num, result) in csv_reader.into_records().enumerate() {
                let record = result?;
                if record.is_empty() || record.iter().all(|field| field.trim().is_empty()) {
                    continue;
                }
                let transaction = Self::parse_record(&record, line_num + 2)?;
                transactions.push(transaction);
            }
            Ok(transactions)
        }
    }

    /// Writes transactions to different targets in YPBankCsvFormat 
    pub fn write<W: std::io::Write>(transactions: &[Transaction], writer: &mut W,) -> Result<(), ParseError> {
        let mut csv_writer = WriterBuilder::new()
            .has_headers(false)
            .flexible(false)
            .quote_style(csv::QuoteStyle::Necessary)
            .from_writer(writer);
        csv_writer.write_record(REQUIRED_FIELDS)?;
        for transaction in transactions {
            if transaction.amount <= 0 {
                return Err(ParseError::InvalidAmount(transaction.amount))
            }
            csv_writer.serialize(&transaction)?
        }
        csv_writer.flush()?;
        Ok(())
    }

    /// Checks if csv headers are valid
    fn validate_headers(headers: &StringRecord) -> Result<(), ParseError> {
        let required_fields: Vec<String> = REQUIRED_FIELDS.iter().map(|s| s.to_string()).collect();
        let actual_fields: Vec<String> = headers.iter().map(|s| s.trim().to_string()).collect();
        if actual_fields.len() != required_fields.len() {
            return Err(ParseError::InvalidHeader(actual_fields, required_fields));
        }
        for (expected, actual) in required_fields.iter().zip(actual_fields.iter()) {
            if expected != actual {
                return Err(ParseError::InvalidHeader(actual_fields, required_fields));
            }
        }
        Ok(())
    }

    /// Parses a single record to return a transaction
    fn parse_record(record: &StringRecord, line_num: usize) -> Result<Transaction, ParseError> {
        let required_fields: Vec<String> = REQUIRED_FIELDS.iter().map(|s| s.to_string()).collect();
        let actual_fields: Vec<String> = record.iter().map(|s| s.trim().to_string()).collect();
        if record.len() != required_fields.len() {
            return Err(ParseError::InvalidRecord(actual_fields, required_fields));
        }
        let tx_id = parse_number("TX_ID", &record[0], line_num)?;
        let tx_type = parse_transaction_type(&record[1], line_num)?;
        let from_user_id = parse_number("FROM_USER_ID", &record[2], line_num)?;
        let to_user_id = parse_number("TO_USER_ID", &record[3], line_num)?;
        let amount = parse_number("AMOUNT", &record[4], line_num)?;
        if amount <= 0 {
            return Err(ParseError::InvalidAmount(amount))
        }
        let timestamp = parse_number("TIMESTAMP", &record[5], line_num)?;
        let status = parse_transaction_status(&record[6], line_num)?;
        let description = (&record[7]).to_string();
        let transaction = Transaction {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description
        };
        transaction.validate()?;
        Ok(transaction)
    }
}