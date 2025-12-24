use crate::errors::ParseError;
use crate::model::transaction::Transaction;
use crate::model::transaction_status::TransactionStatus;
use crate::model::transaction_type::TransactionType;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::ErrorKind;
use std::io::{Read, Write};

pub const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];
const FIXED_RECORD_SIZE: usize = 42;

pub struct YPBankBinParser;
impl YPBankBinParser {
    /// Parses data in YPBankBinFormat from different sources and return transactions
    ///
    /// Checks if data conforms to format requirements
    pub fn parse<R: Read>(reader: &mut R) -> Result<Vec<Transaction>, ParseError> {
        let mut transactions = Vec::new();
        loop {
            match Self::parse_record(reader) {
                Ok(Some(transaction)) => transactions.push(transaction),
                Ok(None) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(transactions)
    }
    /// Writes transactions to different targets in YPBankBinFormat
    pub fn write<W: Write>(transactions: &[Transaction], writer: &mut W) -> Result<(), ParseError> {
        for transaction in transactions {
            writer.write_all(&MAGIC)?;
            let desc_len = transaction.description.len() as u32;
            let record_size = FIXED_RECORD_SIZE as u32 + desc_len;
            writer.write_u32::<BigEndian>(record_size)?;
            writer.write_u64::<BigEndian>(transaction.tx_id)?;
            let tx_type_byte = transaction.tx_type.to_u8();
            writer.write_all(&[tx_type_byte])?;
            writer.write_u64::<BigEndian>(transaction.from_user_id)?;
            writer.write_u64::<BigEndian>(transaction.to_user_id)?;
            let mut amount = transaction.amount;
            if amount > 0 && transaction.tx_type == TransactionType::Withdrawal {
                amount = -amount;
            }
            writer.write_i64::<BigEndian>(amount)?;
            writer.write_u64::<BigEndian>(transaction.timestamp)?;
            writer.write_all(&[transaction.status.to_u8()])?;
            writer.write_u32::<BigEndian>(desc_len)?;
            if desc_len > 0 {
                writer.write_all(transaction.description.as_bytes())?;
            }
        }
        writer.flush()?;
        Ok(())
    }

    /// Parses a single record to return a transaction
    fn parse_record<R: Read>(reader: &mut R) -> Result<Option<Transaction>, ParseError> {
        let mut magic_buf = [0u8; 4];
        match reader.read_exact(&mut magic_buf) {
            Ok(()) => {
                if magic_buf != MAGIC {
                    return Err(ParseError::InvalidMagic(magic_buf, MAGIC));
                }
            }
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                return Ok(None);
            }
            Err(e) => return Err(ParseError::Io(e)),
        }
        let record_size = reader.read_u32::<BigEndian>()?;
        let tx_id = reader.read_u64::<BigEndian>()?;
        let mut tx_type_buf = [0u8; 1];
        reader.read_exact(&mut tx_type_buf)?;
        let tx_type = TransactionType::from_u8(tx_type_buf[0])?;
        let from_user_id = reader.read_u64::<BigEndian>()?;
        let to_user_id = reader.read_u64::<BigEndian>()?;
        let mut amount = reader.read_i64::<BigEndian>()?;
        if amount > 0 && tx_type == TransactionType::Withdrawal {
            return Err(ParseError::InvalidAmountForTransactionType(amount, tx_type));
        } else if amount < 0 && tx_type != TransactionType::Withdrawal {
            return Err(ParseError::InvalidAmountForTransactionType(amount, tx_type));
        }
        if amount < 0 {
            amount = -amount;
        }
        let timestamp = reader.read_u64::<BigEndian>()?;
        let mut status_buf = [0u8; 1];
        reader.read_exact(&mut status_buf)?;
        let status = TransactionStatus::from_u8(status_buf[0])?;
        let desc_len = reader.read_u32::<BigEndian>()?;
        let description = if desc_len > 0 {
            let mut desc_buf = vec![0u8; desc_len as usize];
            reader.read_exact(&mut desc_buf)?;
            String::from_utf8(desc_buf)
                .map_err(|e| ParseError::InvalidDescription(e.to_string()))?
        } else {
            String::new()
        };
        let expected_size = FIXED_RECORD_SIZE as u32 + desc_len;
        if record_size != expected_size {
            return Err(ParseError::InvalidData(format!(
                "Invalid data size: expected {}, actual {}",
                expected_size, record_size
            )));
        }
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
        Ok(Some(transaction))
    }
}