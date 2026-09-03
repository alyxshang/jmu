/*
JMU by Alyx Shang.
Licensed under the FSL v1.
*/

#![deny(clippy::all)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::box_collection)]
#![allow(clippy::redundant_field_names)]

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing the structure
/// to catch and handle errors.
pub use modules::err::*;

/// Re-exporting the module
/// containing the function
/// to split a string of
/// JMU markup into tokens.
pub use modules::lexer::*;

/// Re-exporting the module
/// containing the structure
/// to parse a stream of JMU
/// tokens.
pub use modules::parser::*;

/// Re-exporting the module
/// containing the structure
/// and function to deserialize
/// a string of JMU markup into
/// a Rust data structure that 
/// implements the `Deserialize`
/// trait.
pub use modules::deserialize::*;
