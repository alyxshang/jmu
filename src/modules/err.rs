/*
JMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the namespace
/// for "serde's" "Error" trait.
use serde::de;

/// Importing the "Result"
/// type because it is needed
/// by the "Display" trait.
use std::fmt::Result;

/// Importing the "Display"
/// trait to implement it.
use std::fmt::Display;

/// Importing the "Error"
/// trait to implement it.
use std::error::Error;

/// Importing the "Formatter"
/// structure because it is needed
/// by the "Display" trait.
use std::fmt::Formatter;

/// A structure to catch and handle
/// errors for JMU-related tasks.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct JMUErr {
    pub details: String
}

/// Implementing functions
/// for the `JMUErr` structure.
impl JMUErr {

    /// A function to create
    /// a new instance of the
    /// `JMUErr` structure and
    /// return it.
    pub fn new(
        details: &str
    ) -> JMUErr {
        JMUErr {
            details: details.to_owned()
        }
    }

}

/// Implementing the
/// `Error` trait for
/// the `JMUErr` structure.
impl Error for JMUErr {

    /// The function that
    /// implements the
    /// `Error` trait for
    /// the `JMUErr` structure.
    fn description(
        &self
    ) -> &str {
        &self.details
    }
}

/// Implementing the
/// `Display` trait for
/// the `JMUErr` structure.
impl Display for JMUErr {

    /// The function that
    /// implements the
    /// `Display` trait for
    /// the `JMUErr` structure.
    fn fmt(
        &self, 
        f: &mut Formatter
    ) -> Result {
        write!(f,"{}",self.details)
    }
}

/// Implementing the
/// `de::Error` trait for
/// the `JMUErr` structure.
impl de::Error for JMUErr {

    /// The function that
    /// implements the
    /// `de::Error` trait for
    /// the `JMUErr` structure.
    fn custom<T: Display>(
        msg: T
    ) -> Self {
        JMUErr::new(
            &msg.to_string()
        ) 
    }
}
