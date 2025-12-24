#[cfg(test)]
mod tests {
    use financial_parser::model::transaction_status::TransactionStatus;
    use financial_parser::model::transaction_type::TransactionType;
    use financial_parser::format::yp_bank_bin_format::MAGIC;
    use financial_parser::format::yp_bank_bin_format::YPBankBinParser;
    use std::io::Cursor;
    use financial_parser::model::transaction::Transaction;

    #[test]
    fn test_parse_valid_yp_bank_bin() {
        let mut data = Vec::new();
        data.extend_from_slice(&MAGIC);
        data.extend_from_slice(&51u32.to_be_bytes());
        data.extend_from_slice(&1001u64.to_be_bytes());
        data.push(0);
        data.extend_from_slice(&0u64.to_be_bytes());
        data.extend_from_slice(&501u64.to_be_bytes());
        data.extend_from_slice(&50000i64.to_be_bytes());
        data.extend_from_slice(&1672531200000u64.to_be_bytes());
        data.push(0);
        data.extend_from_slice(&9u32.to_be_bytes());
        data.extend_from_slice(b"Test desc");
        let mut cursor = Cursor::new(data);
        let transactions = YPBankBinParser::parse(&mut cursor).unwrap();
        assert_eq!(transactions.len(), 1);
        let tx = &transactions[0];
        assert_eq!(tx.tx_id, 1001);
        assert_eq!(tx.tx_type, TransactionType::Deposit);
        assert_eq!(tx.from_user_id, 0);
        assert_eq!(tx.to_user_id, 501);
        assert_eq!(tx.amount, 50000);
        assert_eq!(tx.timestamp, 1672531200000);
        assert_eq!(tx.status, TransactionStatus::Success);
        assert_eq!(tx.description, "Test desc");
    }
    
    /// Checks if amount is incorrect for chosen transaction type
    #[test]
    fn test_parse_invalid_amount() {
        let mut data = Vec::new();
        data.extend_from_slice(&MAGIC);
        data.extend_from_slice(&51u32.to_be_bytes());
        data.extend_from_slice(&1001u64.to_be_bytes());
        data.push(0);
        data.extend_from_slice(&0u64.to_be_bytes());
        data.extend_from_slice(&501u64.to_be_bytes());
        data.extend_from_slice(&(-50000i64).to_be_bytes());
        data.extend_from_slice(&1672531200000u64.to_be_bytes());
        data.push(0);
        data.extend_from_slice(&9u32.to_be_bytes());
        data.extend_from_slice(b"Test desc");
        let mut cursor = Cursor::new(data);
        let transactions = YPBankBinParser::parse(&mut cursor);
        assert!(transactions.is_err());
    }

    /// Checks if serialized transaction conforms to YPBankBinFormat
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
        YPBankBinParser::write(&transactions, &mut output).unwrap();
        let mut cursor = Cursor::new(output);
        let transactions_parsed_back = YPBankBinParser::parse(&mut cursor).unwrap();
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