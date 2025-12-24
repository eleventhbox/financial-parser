use crate::errors::ParseError;
use crate::format::Format;
use crate::format::yp_bank_text_format::YPBankTextParser;
use crate::format::yp_bank_csv_format::YPBankCsvParser;
use crate::format::yp_bank_bin_format::YPBankBinParser;
use crate::model::transaction::Transaction;

pub struct Parser;
impl Parser {
    /// Library entry point for data parsing from different sources
    pub fn parse<R: std::io::Read>(mut reader: R, format: Format) -> Result<Vec<Transaction>, ParseError> {
        let transactions = match format {
            Format::Text => YPBankTextParser::parse(reader)?,
            Format::Csv => YPBankCsvParser::parse(reader)?,
            Format::Binary => YPBankBinParser::parse(&mut reader)?
        };
        Ok(transactions)
    }

    /// Library entry point for data writing to different targets 
    pub fn write<W: std::io::Write>(transactions: &[Transaction], mut writer: W, format: Format,) -> Result<(), ParseError> {
        match format {
            Format::Text => YPBankTextParser::write(transactions, &mut writer, true),
            Format::Csv => YPBankCsvParser::write(transactions, &mut writer),
            Format::Binary => YPBankBinParser::write(transactions, &mut writer)
        }
    }
    
}