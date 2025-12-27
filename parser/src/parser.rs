use crate::errors::ParseError;
use crate::format::Format;
use crate::format::yp_bank_text_format::YPBankTextParser;
use crate::format::yp_bank_csv_format::YPBankCsvParser;
use crate::format::yp_bank_bin_format::YPBankBinParser;
use crate::model::transaction::Transaction;

/// Parser
pub struct Parser;
impl Parser {
    /// Parses data from different sources.
    ///
    /// # Parameters
    ///
    /// * `reader` — any type, implementing `std::io::Read`, from which data can be read
    ///
    /// # Returning value
    ///
    /// Returns `Result<Vec<Transaction>, ParseError>`:
    /// - `Ok(Vec<Transaction>)` — successful parsing, contains transaction vector
    /// - `Err(ParseError)` — parsing error (I/O, CSV, validation etc.)
    pub fn parse<R: std::io::Read>(mut reader: R, format: Format) -> Result<Vec<Transaction>, ParseError> {
        let transactions = match format {
            Format::Text => YPBankTextParser::parse(reader)?,
            Format::Csv => YPBankCsvParser::parse(reader)?,
            Format::Binary => YPBankBinParser::parse(&mut reader)?
        };
        Ok(transactions)
    }

    /// Writes transaction vector into chosen sink.
    ///
    /// Function serializes every transaction into chosen format and writes it into `writer`.
    ///
    /// # Parameters
    ///
    /// * `transactions` — transaction slice to write
    /// * `writer` — any type, implementing `std::io::Write`, into which CSV data will be written
    /// * `format` - format to write data into
    ///
    /// # Returning value
    ///
    /// Returns `Result<(), ParseError>`:
    /// - `Ok(())` — all transactions successfully written
    /// - `Err(ParseError)` — write or validation error 
    pub fn write<W: std::io::Write>(transactions: &[Transaction], mut writer: W, format: Format,) -> Result<(), ParseError> {
        match format {
            Format::Text => YPBankTextParser::write(transactions, &mut writer, true),
            Format::Csv => YPBankCsvParser::write(transactions, &mut writer),
            Format::Binary => YPBankBinParser::write(transactions, &mut writer)
        }
    }
    
}