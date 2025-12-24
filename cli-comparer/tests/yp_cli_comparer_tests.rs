#[cfg(test)]
mod tests {
    use financial_parser::format::common::prepare_transactions;
    use financial_parser::format::Format;
    use financial_parser::model::transaction::Transaction;
    use financial_parser::model::transaction_status::TransactionStatus;
    use financial_parser::model::transaction_type::TransactionType;
    use financial_parser::parser::Parser;
    use std::io::Write;
    use tempfile::NamedTempFile;
    #[test]
    fn test_compare_files() -> Result<(), Box<dyn std::error::Error>> {
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
        let mut file1 = NamedTempFile::new()?;
        Parser::write(&transactions, file1.as_file_mut(), Format::Csv)?;
        file1.flush()?;
        let mut file2 = NamedTempFile::new()?;
        Parser::write(&transactions, file2.as_file_mut(), Format::Binary)?;
        file2.flush()?;
        let result = prepare_transactions(
            &file1.path().to_path_buf(),
            Format::Csv,
            &file2.path().to_path_buf(),
            Format::Binary,
        )?;
        assert_eq!(result.0, result.1);
        Ok(())
    }
}