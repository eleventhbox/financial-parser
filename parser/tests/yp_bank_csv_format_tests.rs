#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use indoc::indoc;
    use financial_parser::format::yp_bank_csv_format::YPBankCsvParser;
    use financial_parser::model::transaction::Transaction;
    use financial_parser::model::transaction_type::TransactionType;
    use financial_parser::model::transaction_status::TransactionStatus;

    const SAMPLE_YP_BANK_CSV: &str = indoc! {r#"
        TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
        1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding"
        1002,TRANSFER,501,502,15000,1672534800000,FAILURE,"Payment for services, invoice #123"
        1003,WITHDRAWAL,502,0,1000,1672538400000,PENDING,"ATM withdrawal"
    "#};

    /// Check if provided csv data is valid YPBankCsvFormat
    #[test]
    fn test_parse_valid_yp_bank_csv() {
        let cursor = Cursor::new(SAMPLE_YP_BANK_CSV);
        let result = YPBankCsvParser::parse(cursor);
        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 3);
        let deposit = &transactions[0];
        assert_eq!(deposit.tx_id, 1001);
        assert_eq!(deposit.tx_type, TransactionType::Deposit);
        assert_eq!(deposit.from_user_id, 0);
        assert_eq!(deposit.to_user_id, 501);
        assert_eq!(deposit.amount, 50000);
        assert_eq!(deposit.timestamp, 1672531200000);
        assert_eq!(deposit.status, TransactionStatus::Success);
        assert_eq!(deposit.description, "Initial account funding");
        let transfer = &transactions[1];
        assert_eq!(transfer.tx_type, TransactionType::Transfer);
        assert_eq!(transfer.status, TransactionStatus::Failure);
        let withdrawal = &transactions[2];
        assert_eq!(withdrawal.tx_type, TransactionType::Withdrawal);
        assert_eq!(withdrawal.to_user_id, 0);
    }

    /// Checks if serialized transaction conforms to YPBankCsvFormat
    #[test]
    fn test_serialize_yp_bank_csv() {
        let transaction = Transaction {
            tx_id: 123,
            tx_type: TransactionType::Deposit,
            from_user_id: 0,
            to_user_id: 456,
            amount: 100,
            timestamp: 1633036800000,
            status: TransactionStatus::Success,
            description: "Test".to_string(),
        };
        let transactions= vec![transaction]; 
        let mut output = Vec::new();
        YPBankCsvParser::write(&transactions, &mut output).unwrap();
        let output_str = String::from_utf8(output.clone()).unwrap();
        let new_str = output_str.clone();
        let first_line = new_str.lines().next().unwrap();
        assert_eq!(first_line, "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION");
        let cursor = Cursor::new(output_str);
        let transactions_parsed_back = YPBankCsvParser::parse(cursor).unwrap();
        assert_eq!(transactions.len(), transactions_parsed_back.len());
        for (orig, parsed) in transactions.iter().zip(transactions_parsed_back.iter()) {
            assert_eq!(orig.tx_id, parsed.tx_id);
            assert_eq!(orig.tx_type, parsed.tx_type);
            assert_eq!(orig.from_user_id, parsed.from_user_id);
            assert_eq!(orig.to_user_id, parsed.to_user_id);
            assert_eq!(orig.amount, parsed.amount);
            assert_eq!(orig.timestamp, parsed.timestamp);
            assert_eq!(orig.status, parsed.status);
            assert_eq!(orig.description, parsed.description);
        }
    }
}