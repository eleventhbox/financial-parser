#[cfg(test)]
mod tests {
    use financial_parser::errors::ParseError;
    use std::io::Cursor;
    use indoc::indoc;
    use financial_parser::format::yp_bank_text_format::YPBankTextParser;
    use financial_parser::model::transaction::Transaction;
    use financial_parser::model::transaction_type::TransactionType;
    use financial_parser::model::transaction_status::TransactionStatus;

    const SAMPLE_YP_BANK_TEXT: &str = indoc! {r#"
        # Record 1 (Deposit)
        TX_ID: 1234567890123456
        TX_TYPE: DEPOSIT
        FROM_USER_ID: 0
        TO_USER_ID: 9876543210987654
        AMOUNT: 10000
        TIMESTAMP: 1633036800000
        STATUS: SUCCESS
        DESCRIPTION: "Terminal deposit"

        # Record 2 (Transfer)
        TX_ID: 2312321321321321
        TIMESTAMP: 1633056800000
        STATUS: FAILURE
        TX_TYPE: TRANSFER
        FROM_USER_ID: 1231231231231231
        TO_USER_ID: 9876543210987654
        AMOUNT: 1000
        DESCRIPTION: "User transfer"
        
        # Record 3 (Withdrawal)
        TX_ID: 3213213213213213
        AMOUNT: 100
        TX_TYPE: WITHDRAWAL
        FROM_USER_ID: 9876543210987654
        TO_USER_ID: 0
        TIMESTAMP: 1633066800000
        STATUS: SUCCESS
        DESCRIPTION: "User withdrawal"
    "#};

    /// Check if provided text data is valid YPBankTextFormat
    /// 
    /// Fields within record can be placed in no particular order
    /// 
    /// Records separated with empty string
    /// 
    /// Lines started with # sign are ignored
    #[test]
    fn test_parse_valid_yp_bank_text() {
        let cursor = Cursor::new(SAMPLE_YP_BANK_TEXT);
        let result = YPBankTextParser::parse(cursor);
        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 3);
        let deposit = &transactions[0];
        assert_eq!(deposit.tx_id, 1234567890123456);
        assert_eq!(deposit.tx_type, TransactionType::Deposit);
        assert_eq!(deposit.from_user_id, 0);
        assert_eq!(deposit.to_user_id, 9876543210987654);
        assert_eq!(deposit.amount, 10000);
        assert_eq!(deposit.timestamp, 1633036800000);
        assert_eq!(deposit.status, TransactionStatus::Success);
        assert_eq!(deposit.description, "Terminal deposit");
        let transfer = &transactions[1];
        assert_eq!(transfer.tx_type, TransactionType::Transfer);
        assert_eq!(transfer.status, TransactionStatus::Failure);
        let withdrawal = &transactions[2];
        assert_eq!(withdrawal.tx_type, TransactionType::Withdrawal);
        assert_eq!(withdrawal.to_user_id, 0);
    }

    /// Checks if invalid format data results in Err
    #[test]
    fn test_parse_invalid_format() {
        let invalid_data = "TX_ID: not_a_number\nTX_TYPE: DEPOSIT\n";
        let cursor = Cursor::new(invalid_data);
        let result = YPBankTextParser::parse(cursor);
        assert!(result.is_err());
    }

    /// Checks if missing fields results in Err
    #[test]
    fn test_parse_missing_field() {
        let missing_field = indoc! {r#"
            TX_ID: 123
            TX_TYPE: DEPOSIT
            FROM_USER_ID: 0
            TO_USER_ID: 456
            AMOUNT: 100
            TIMESTAMP: 1633036800000
            STATUS: SUCCESS
        "#};
        let cursor = Cursor::new(missing_field);
        let result = YPBankTextParser::parse(cursor);
        assert!(result.is_err());
        if let Err(ParseError::Validation(msg)) = result {
            assert!(msg.contains("Missing required field"));
            assert!(msg.contains("DESCRIPTION"));
        } else {
            panic!("Expected YPBankTextParser error");
        }
    }

    /// Checks if duplicate fields results in Err
    #[test]
    fn test_parse_duplicate_field() {
        let duplicate = indoc! {r#"TX_ID: 123
            TX_TYPE: DEPOSIT
            TX_TYPE: TRANSFER
            FROM_USER_ID: 0
            TO_USER_ID: 456
            AMOUNT: 100
            TIMESTAMP: 1633036800000
            STATUS: SUCCESS
            DESCRIPTION: "Test"
        "#};
        let cursor = Cursor::new(duplicate);
        let result = YPBankTextParser::parse(cursor);
        assert!(result.is_err());
        if let Err(ParseError::Validation(msg)) = result {
            assert!(msg.contains("Duplicate key"));
        } else {
            panic!("Expected YPBankTextParser error");
        }
    }
    
    /// Checks if DESCRIPTION field with no double quotes results in Err
    #[test]
    fn test_parse_description_without_quotes() {
        let no_quotes = indoc! {r#"
            TX_ID: 123
            TX_TYPE: DEPOSIT
            FROM_USER_ID: 0
            TO_USER_ID: 456
            AMOUNT: 100
            TIMESTAMP: 1633036800000
            STATUS: SUCCESS
            DESCRIPTION: No quotes here
        "#};
        let cursor = Cursor::new(no_quotes);
        let result = YPBankTextParser::parse(cursor);
        assert!(result.is_err());
        if let Err(ParseError::Validation(msg)) = result {
            assert!(msg.contains("must be in double quotes"));
        } else {
            panic!("Expected YPBankTextParser error");
        }
    }

    /// Checks if transaction requirements violation results in Err
    #[test]
    fn test_validation() {
        let transaction = Transaction {
            tx_id: 123,
            tx_type: TransactionType::Deposit,
            from_user_id: 1,
            to_user_id: 456,
            amount: 100,
            timestamp: 1633036800000,
            status: TransactionStatus::Success,
            description: "Test".to_string(),
        };
        let result = transaction.validate();
        assert!(result.is_err());
        if let Err(ParseError::Validation(msg)) = result {
            assert!(msg.contains("must be 0 for DEPOSIT"));
        } else {
            panic!("Expected Validation error");
        }
    }

    /// Checks if serialized transaction conforms to YPBankTextFormat
    #[test]
    fn test_serialize_yp_bank_text() {
        let transactions = vec![
            Transaction {
                tx_id: 1234567890123456,
                tx_type: TransactionType::Deposit,
                from_user_id: 0,
                to_user_id: 9876543210987654,
                amount: 10000,
                timestamp: 1633036800000,
                status: TransactionStatus::Success,
                description: "Terminal deposit".to_string(),
            },
        ];
        let mut output = Vec::new();
        YPBankTextParser::write(&transactions, &mut output, true).unwrap();
        let cursor = Cursor::new(output);
        let transactions_parsed_back = YPBankTextParser::parse(cursor).unwrap();
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