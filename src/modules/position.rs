/*
JMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// A structure to record
/// the position of a character
/// in source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize
}

/// Implementing the `new`
/// function for the `Position`
/// structure.
impl Position {

    /// A function to create a new
    /// instance of the `Position`
    /// structure and return the
    /// created instance.
    pub fn new(
        line: &usize,
        column: &usize
    ) -> Position {
        Position {
            line: *line,
            column: *column
        }
    }
}
