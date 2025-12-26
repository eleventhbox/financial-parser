#![warn(missing_docs)]
//! # financial-parser library
//! 
//! This library contains functionality for different financial transaction formats parsing 

/// # Error processing module
///
/// This module contains errors, which can be raised in
/// `financial-parser` library when parsing different format data
/// and validating transactions.
pub mod errors;

/// # Acceptable formats module
/// 
/// This module contains the list of acceptable formats
pub mod format;

/// # Models module
/// 
/// This module contains model entities used when reading and writing data
pub mod model;

/// # Parser module
/// 
/// This module is responsible for dispatching reading and writing to different parsers
/// depending on the chosen format
pub mod parser;
