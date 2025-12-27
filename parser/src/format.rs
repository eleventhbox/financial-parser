/// # YPBankTextFormat parser module
/// 
/// This module contains functionality for reading and writing data in YPBankText format
pub mod yp_bank_text_format;
/// # YPBankBinFormat parser module
///
/// This module contains functionality for reading and writing data in YPBankBin format
pub mod yp_bank_bin_format;
/// # YPBankCSVFormat parser module
///
/// This module contains functionality for reading and writing data in YPBankCsv format
pub mod yp_bank_csv_format;
/// # Common functionality module
///
/// This module contains functionality common for several parsers
pub mod common;

use clap::ValueEnum;
use strum_macros::{EnumString, Display};

/// Acceptable parsing formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[derive(EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Format {
    /// YPBankText format
    Text, 
    /// YPBankCsv format
    Csv, 
    /// YPBankBin format
    Binary 
}