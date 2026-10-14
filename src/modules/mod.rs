/*
JMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// containing the structure
/// to catch and handle errors.
pub mod err;

/// Exporting the module
/// containing the function
/// to split a string of
/// JMU markup into tokens.
pub mod lexer;

/// Declaring the module
/// containing this crate's
/// unit tests.
#[cfg(test)]
pub mod tests;

/// Exporting the module
/// containing the structure
/// to parse a stream of JMU
/// tokens.
pub mod parser;

/// Exporting the module
/// containing the structure
/// to record position info
/// on a character.
pub mod position;

/// Exporting the module
/// containing the structure
/// and function to deserialize
/// a string of JMU markup into
/// a Rust data structure that 
/// implements the `Deserialize`
/// trait.
pub mod deserialize;
