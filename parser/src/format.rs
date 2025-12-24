pub mod yp_bank_text_format;
pub mod yp_bank_bin_format;
pub mod yp_bank_csv_format;
pub mod common;

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Format { Text, Csv, Binary }

impl std::str::FromStr for Format {
    type Err = String;

    /// Format variant from &str
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Format::Text),
            "csv" => Ok(Format::Csv),
            "binary" => Ok(Format::Binary),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

