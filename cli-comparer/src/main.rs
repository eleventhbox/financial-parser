use clap::Parser as ClapParser;
use financial_parser::format::common::prepare_transactions;
use financial_parser::format::Format;
use std::path::PathBuf;

#[derive(Debug, ClapParser)]
#[command(name = "cli-comparer")]
#[command(about = "Compare transaction records from two files", long_about = None)]
#[command(version)]
struct Args {
    #[arg(short = '1', long = "file1", required = true)]
    file1: PathBuf,
    #[arg(
        short = 'f',
        long = "format1",
        default_value = "csv",
        help = "Input format for file1: csv, text, binary"
    )]
    format1: Format,
    #[arg(short = '2', long = "file2", required = true)]
    file2: PathBuf,
    #[arg(
        short = 'F',
        long = "format2",
        default_value = "csv",
        help = "Input format for file2: csv, text, binary"
    )]
    format2: Format,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let result = prepare_transactions(&args.file1, args.format1, &args.file2, args.format2)?;
    if result.0 == result.1 {
        println!(
            "The transaction records in '{}' and '{}' are identical.",
            args.file1.display(),
            args.file2.display()
        );
    } else {
        let mismatch = result
            .0
            .iter()
            .zip(result.1.iter())
            .position(|(a, b)| a != b);
        if let Some(index) = mismatch {
            eprintln!(
                "Mismatch at transaction index {}: {:?} vs {:?}",
                index,
                result.0.get(index),
                result.1.get(index)
            );
        } else if result.0.len() != result.1.len() {
            eprintln!(
                "Transaction lists have different lengths: {} vs {}",
                result.0.len(),
                result.1.len()
            );
        }
        eprintln!(
            "The transaction records in '{}' and '{}' do not match.",
            args.file1.display(),
            args.file2.display()
        );
    }
    Ok(())
}
