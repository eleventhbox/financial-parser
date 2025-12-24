use financial_parser::model::transaction_status::TransactionStatus;
use financial_parser::model::transaction_type::TransactionType;
use std::io::Cursor;
use tempfile::NamedTempFile;
use std::fs;
use std::process::Command;
use std::io::Write;
use financial_parser::parser::Parser;
use financial_parser::format::Format;
use financial_parser::model::transaction::Transaction;

#[test]
fn test_cli_converter_csv_to_binary() -> Result<(), Box<dyn std::error::Error>> {
    let transactions = vec![
        Transaction {
            tx_id: 1001,
            tx_type: TransactionType::Deposit,
            from_user_id: 0,
            to_user_id: 501,
            amount: 50000,
            timestamp: 1672531200000,
            status: TransactionStatus::Success,
            description: "Initial deposit".to_string(),
        },
        Transaction {
            tx_id: 1002,
            tx_type: TransactionType::Transfer,
            from_user_id: 501,
            to_user_id: 502,
            amount: 15000,
            timestamp: 1672534800000,
            status: TransactionStatus::Failure,
            description: "Payment".to_string(),
        }
    ];

    let mut input_file = NamedTempFile::new()?;
    Parser::write(&transactions, input_file.as_file_mut(), Format::Csv)?;
    input_file.flush()?;
    let output_file = NamedTempFile::new()?;
    let status = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cli-converter",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--input-format",
            "csv",
            "--output",
            output_file.path().to_str().unwrap(),
            "--output-format",
            "binary",
        ])
        .status()?;
    assert!(status.success());
    let output_bytes = fs::read(output_file.path())?;
    let mut cursor = Cursor::new(output_bytes);
    let parsed_transactions = Parser::parse(&mut cursor, Format::Binary)?;
    assert_eq!(transactions, parsed_transactions);
    Ok(())
}

#[test]
fn test_cli_converter_csv_to_text_to_csv() -> Result<(), Box<dyn std::error::Error>> {
    let transactions = vec![
        Transaction {
            tx_id: 1001,
            tx_type: TransactionType::Deposit,
            from_user_id: 0,
            to_user_id: 501,
            amount: 50000,
            timestamp: 1672531200000,
            status: TransactionStatus::Success,
            description: "Initial deposit".to_string(),
        }
    ];
    let mut input_file = NamedTempFile::new()?;
    Parser::write(&transactions, input_file.as_file_mut(), Format::Csv)?;
    input_file.flush()?;
    let intermediate_file = NamedTempFile::new()?;
    let final_output_file = NamedTempFile::new()?;
    let status1 = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cli-converter",
            "--",
            "--input",
            input_file.path().to_str().unwrap(),
            "--input-format",
            "csv",
            "--output",
            intermediate_file.path().to_str().unwrap(),
            "--output-format",
            "text",
        ])
        .status()?;
    assert!(status1.success());
    let status2 = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cli-converter",
            "--",
            "--input",
            intermediate_file.path().to_str().unwrap(),
            "--input-format",
            "text",
            "--output",
            final_output_file.path().to_str().unwrap(),
            "--output-format",
            "csv",
        ])
        .status()?;
    assert!(status2.success());
    let final_bytes = fs::read(final_output_file.path())?;
    let mut cursor = Cursor::new(final_bytes);
    let final_transactions = Parser::parse(&mut cursor, Format::Csv)?;
    assert_eq!(transactions, final_transactions);
    Ok(())
}