use clap::Parser as ClapParser;
use financial_parser::format::Format;
use financial_parser::parser::Parser;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

#[derive(Debug, ClapParser)]
#[command(name = "cli-converter")]
#[command(about = "Transaction converter for different formats", long_about = None)]
#[command(version)]
struct Args {
    #[arg(short, long, help = "Input file")]
    input: PathBuf,
    #[arg(
        short = 'f',
        long = "input-format",
        default_value = "csv",
        help = "Input format: csv, text, or binary"
    )]
    input_format: Format,
    #[arg(short, long, help = "Output file")]
    output: PathBuf,
    #[arg(
        short = 'F',
        long = "output-format",
        default_value = "csv",
        help = "Output format: csv, text, or binary"
    )]
    output_format: Format,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = File::open(&args.input)
        .map_err(|e| format!("File can not be opened {}: {}", &args.input.display(), e))?;
    let mut input_reader: Box<dyn Read> = Box::new(BufReader::new(file));
    let transactions = Parser::parse(&mut input_reader, args.input_format)
        .map_err(|e| format!("Parsing error: {}", e))?;
    let file = File::create(&args.output)
        .map_err(|e| format!("File can not be created {}: {}", &args.output.display(), e))?;
    let mut output_writer: Box<dyn Write> = Box::new(BufWriter::new(file));
    Parser::write(&transactions, &mut output_writer, args.output_format)
        .map_err(|e| format!("Write error: {}", e))?;
    Ok(())
}