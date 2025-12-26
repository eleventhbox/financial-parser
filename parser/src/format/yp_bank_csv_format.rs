use crate::format::yp_bank_csv_format::Idx::{
    TxId, TxType, FromUserId, ToUserId,
    Amount, Timestamp, Status, Description
};
use crate::errors::ParseError;
use crate::format::common::{parse_number, parse_transaction_status, parse_transaction_type};
use crate::model::transaction::Transaction;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::io::Read;

const REQUIRED_FIELDS: &[&str] = &[
    "TX_ID", "TX_TYPE", "FROM_USER_ID", "TO_USER_ID",
    "AMOUNT", "TIMESTAMP", "STATUS", "DESCRIPTION"
];


#[derive(Debug, Clone, Copy)]
enum Idx {
    TxId,
    TxType,
    FromUserId,
    ToUserId,
    Amount,
    Timestamp,
    Status,
    Description
}

impl From<Idx> for usize {
    fn from(idx: Idx) -> Self {
        match idx {
            TxId => 0,
            TxType => 1,
            FromUserId => 2,
            ToUserId => 3,
            Amount => 4,
            Timestamp => 5,
            Status => 6,
            Description => 7,
        }
    }
}

/// Reading and writing data in YPBankCsv format
pub struct YPBankCsvParser;
impl YPBankCsvParser {
    /// Parses data in YPBankCsv format from different sources.
    ///
    /// Reads a CSV file, validates headers and every record,
    /// transforming them into transaction vector. Empty lines are ignored.
    ///
    /// # Parameters
    ///
    /// * `reader` — any type, implementing `std::io::Read`, from which CSV data can be read
    ///
    /// # Returning value
    ///
    /// Returns `Result<Vec<Transaction>, ParseError>`:
    /// - `Ok(Vec<Transaction>)` — successful parsing, contains transaction vector
    /// - `Err(ParseError)` — parsing error (I/O, CSV, validation etc.)
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
    
    /// Writes transaction vector into chosen sink in YPBankCsv format.
    ///
    /// Function serializes every transaction into CSV record and writes it into `writer`.
    /// Also writes header, defined in `REQUIRED_FIELDS`.
    ///
    /// # Parameters
    ///
    /// * `transactions` — transaction slice to write
    /// * `writer` — any type, implementing `std::io::Write`, into which CSV data will be written
    ///
    /// # Returning value
    ///
    /// Returns `Result<(), ParseError>`:
    /// - `Ok(())` — all transactions successfully written
    /// - `Err(ParseError)` — write or validation error
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
            csv_writer.serialize(transaction)?
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
        let tx_id = parse_number("TxId", &record[TxId.into()], line_num)?;
        let tx_type = parse_transaction_type(&record[TxType.into()], line_num)?;
        let from_user_id = parse_number("FromUserId", &record[FromUserId.into()], line_num)?;
        let to_user_id = parse_number("ToUserId", &record[ToUserId.into()], line_num)?;
        let amount = parse_number("AMOUNT", &record[Amount.into()], line_num)?;
        if amount <= 0 {
            return Err(ParseError::InvalidAmount(amount))
        }
        let timestamp = parse_number("TIMESTAMP", &record[Timestamp.into()], line_num)?;
        let status = parse_transaction_status(&record[Status.into()], line_num)?;
        let description = record[Description.into()].to_string();
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