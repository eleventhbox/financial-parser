use crate::errors::ParseError;
use crate::format::Format;
use crate::model::transaction::Transaction;
use crate::model::transaction_status::TransactionStatus;
use crate::model::transaction_type::TransactionType;
use crate::parser::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

pub fn prepare_transactions(
    file1_path: &PathBuf,
    format1: Format,
    file2_path: &PathBuf,
    format2: Format,
) -> Result<(Vec<Transaction>, Vec<Transaction>), Box<dyn std::error::Error>> {
    let mut file1 = BufReader::new(File::open(file1_path)?);
    let mut file2 = BufReader::new(File::open(file2_path)?);
    let transactions1 = Parser::parse(&mut file1, format1)?;
    let transactions2 = Parser::parse(&mut file2, format2)?;
    Ok((transactions1,transactions2))
}

pub fn parse_number<T>(field_name: &str, value: &str, line_number: usize) -> Result<T, ParseError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    value.parse::<T>().map_err(|e| {
        ParseError::Validation(format!(
            "Invalid {} value '{}' at line {}: {}",
            field_name, value, line_number, e
        ))
    })
}

pub fn parse_transaction_type(value: &str, line_number: usize) -> Result<TransactionType, ParseError> {
    TransactionType::from_str(value)
        .map_err(|e| ParseError::Validation(format!(
            "Invalid TX_TYPE '{}' at line {}: {}",
            value, line_number, e
        )))
}

pub fn parse_transaction_status(value: &str, line_number: usize) -> Result<TransactionStatus, ParseError> {
    TransactionStatus::from_str(value)
        .map_err(|e| ParseError::Validation(format!(
            "Invalid STATUS '{}' at line {}: {}",
            value, line_number, e
        )))
}

pub fn parse_description(value: &str, line_number: usize) -> Result<String, ParseError> {
    if !value.starts_with('"') || !value.ends_with('"') {
        return Err(ParseError::Validation(format!(
            "DESCRIPTION must be in double quotes at line {}: {}",
            line_number, value
        )));
    }

    let content = &value[1..value.len() - 1];

    Ok(content.to_string())
}