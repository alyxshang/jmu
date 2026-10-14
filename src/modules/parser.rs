/*
JMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// to catch and handle errors.
use super::err::JMUErr;

/// Importing the structure
/// that encapsulates data
/// about a captured JMU
/// token.
use super::lexer::Token;

/// Importing the enumeration
/// "listing" all possible types
/// of tokens in JMU source
/// code.
use super::lexer::TokenType;

/// An enumeration "listing"
/// all possible types of
/// assignment.
#[derive(PartialEq, Debug)]
pub enum Assignment{
    Comment,
    BoolAssign(String, bool),
    StringAssign(String,String),
    FloatingAssign(String, f64),
    IntegerAssign(String, usize),
    BoolListAssign(String, Vec<bool>),
    FloatingListAssign(String, Vec<f64>),
    IntegerListAssign(String, Vec<usize>),
    StringListAssign(String, Vec<String>),
    NestedAssign(String, Box<Vec<Assignment>>),
    AssignmentList(String, Vec<Box<Vec<Assignment>>>)
}

/// A structure to
/// hold the stream
/// of captured tokens
/// and keep a cursor
/// to keep track of
/// the position in
/// the stream of tokens.
pub struct Parser{
    pub cursor: usize,
    pub stream: Vec<Token>
}

/// Implementing functions
/// for the `Parser` structure.
impl Parser {

    /// A function to create a new
    /// instance of the `Parser`
    /// structure and return it.
    /// If the stream of tokens 
    /// is empty, an error is returned.
    pub fn new(
        stream: &Vec<Token>
    ) -> Result<Parser, JMUErr> {
        if stream.is_empty(){
            Err::<Parser, JMUErr>(
                JMUErr::new("Token stream cannot be empty.")
            )
        }
        else{
            Ok(
                Parser{
                    cursor: 0,
                    stream: stream.clone()
                }
            )
        }
    }

    /// A function to check whether
    /// the cursor has reached the 
    /// end of the stream of tokens.
    /// A boolean is returned to
    /// reflect this.
    pub fn is_done(
        &self
    ) -> bool {
        self.stream.len() == self.cursor
    }

    /// A function to advance
    /// the internal cursor 
    /// by one.
    pub fn advance(
        &mut self
    ){
        self.cursor += 1;
    }

    /// A function to "peek"
    /// ahead at the token
    /// at the current position
    /// incremented by N. If the
    /// operation is successful,
    /// an instance of the `Token`
    /// structure is returned. If
    /// the operation fails, an
    /// error is returned.
    pub fn peek_n(
        &mut self,
        n: &usize
    ) -> Result<Token, JMUErr>{
        if self.is_done(){
            Err::<Token, JMUErr>(
                JMUErr::new("Unexpected end of token stream.")
            )
        }
        else {
            let token: Token = match self.stream.get(self.cursor + n){
                Some(token) => token.clone(),
                None => return Err::<Token, JMUErr>(
                    JMUErr::new("Unexpected end of token stream.")
                )
            };
            Ok(token)
        }
    }

    /// A function to retrieve
    /// the current token in the
    /// stream of tokens and 
    /// advance the cursor. If the
    /// operation is successful,
    /// an instance of the `Token`
    /// structure is returned. If
    /// the operation fails, an
    /// error is returned.
    pub fn expect(
        &mut self,
        token_type: &TokenType
    ) -> Result<Token, JMUErr>{
        let token: Token = match self.stream.get(self.cursor){
            Some(token) => token.clone(),
            None => return Err::<Token, JMUErr>(
                JMUErr::new("Unexpected end of token stream.")
            )
        };
        if token.token_type == token_type.clone(){
            self.advance();
            Ok(token)
        }
        else{
            Err::<Token, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Expected token of type \"{:?}\" at position \"{:?}:{:?}\"!",
                        token_type,
                        token.start.line,
                        token.start.column
                    )
                )
            )
        }
    }

    /// A function to parse the entire internal
    /// stream of tokens and return the parsed
    /// syntax tree as a vector of instances of
    /// the `Assignment` enumeration. If the
    /// operation fails, an error is returned.
    pub fn parse(
        &mut self
    ) -> Result<Vec<Assignment>, JMUErr>{
        let mut assignments: Vec<Assignment> = Vec::new();
        while !self.is_done(){
            assignments.push(self.parse_assignment()?);
        }
        Ok(assignments)
    }

    /// A function to parse a single
    /// assignment. Which type of assignment
    /// is parsed depends on the first token
    /// that is given as the value of an
    /// assignment. If the operation
    /// is successful, an instance of the
    /// `Assignment` enumeration is returned.
    /// If the operation fails, an error is
    /// returned.
    pub fn parse_assignment(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        if self.peek_n(&0)?.token_type == TokenType::UserString &&
            self.peek_n(&1)?.token_type == TokenType::Assign
        {
            let next: Token = self.peek_n(&2)?;
            match next.token_type{
                TokenType::Integer => Ok(
                    self.parse_integer_assign()?
                ),
                TokenType::FloatingPoint => Ok(
                    self.parse_floating_point_assign()?
                ),
                TokenType::UserString => Ok(
                    self.parse_string_assign()?
                ),
                TokenType::Boolean => Ok(
                    self.parse_bool_assign()?
                ),
                TokenType::OpenBracket => Ok(
                    self.parse_list_assign()?
                ),
                TokenType::SectionMarker => Ok(
                    self.parse_nested_assignment()?
                ), 
                _ => Err::<Assignment, JMUErr>(
                    JMUErr::new(
                        &format!(
                            "Unexpected value token at position \"{:?}:{:?}\"!",
                            next.start.line,
                            next.start.column
                        )
                    )
                )
            }
        }
        else if self.peek_n(&0)?.token_type == TokenType::UserComment{
            self.advance();
            Ok(Assignment::Comment)
        }
        else {
            Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Expected a string and then an assignment smybol at position \"{:?}:{:?}\"!",
                        self.peek_n(&0)?.start.line,
                        self.peek_n(&0)?.start.column
                    )
                )
            )
        }
    }

    /// A function to parse an assignment 
    /// to a list of values. This list can 
    /// either be a list of values or 
    /// assignment blocks. If the operation 
    /// is successful, a variant of the 
    /// `Assignment` enumeration of the 
    /// type `*ListAssign` is returned. 
    /// If the operation fails, an error is
    /// returned.
    pub fn parse_list_assign(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        if self.peek_n(&0)?.token_type == TokenType::UserString &&
            self.peek_n(&1)?.token_type == TokenType::Assign &&
            self.peek_n(&2)?.token_type == TokenType::OpenBracket
        {
            let first: Token = self.peek_n(&3)?;
            match first.token_type{
                TokenType::UserString =>
                    Ok(self.parse_string_list_assign()?),
                TokenType::Boolean =>
                    Ok(self.parse_bool_list_assign()?),
                TokenType::Integer =>
                    Ok(self.parse_integer_list_assign()?),
                TokenType::FloatingPoint =>
                    Ok(self.parse_floating_point_list_assign()?),
                TokenType::SectionMarker =>
                    Ok(self.parse_assignment_list()?),
                _ => Err::<Assignment, JMUErr>(
                    JMUErr::new(
                        &format!(
                            "Unexpected token at position \"{:?}:{:?}\"!",
                            self.peek_n(&0)?.start.line,
                            self.peek_n(&0)?.start.column
                        )
                    )
                )
            }
        }
        else {
            Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Expected a string, an assignment symbol, and an open square bracket at position \"{:?}:{:?}\"!",
                        self.peek_n(&0)?.start.line,
                        self.peek_n(&0)?.start.column
                    )

                )
            )
        }
    }

    /// A function to parse an assignment to a 
    /// list of string values. If the operation 
    /// is successful, the `StringListAssign`
    /// variant of the `Assignment`
    /// enumeration is returned. If the
    /// operation fails, an error is
    /// returned.
    pub fn parse_string_list_assign(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key_token: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let _open: Token = self.expect(&TokenType::OpenBracket)?;
        let mut value_buf: Vec<String> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if self.is_done(){
                return Err::<Assignment, JMUErr>(
                    JMUErr::new("Unexpected end of token stream.")
                )
            }
            else if token.token_type == TokenType::CloseBracket{
                self.advance();
                break;
            }
            else {
                let val_token: Token;
                if self.peek_n(&1)?.token_type == TokenType::CloseBracket{
                    val_token = self.expect(&TokenType::UserString)?;
                }
                else {
                    val_token = self.expect(&TokenType::UserString)?;
                    let _comma: Token = self.expect(&TokenType::Comma)?;
                }
                let value: String = match val_token.value{
                    Some(value) => value,
                    None => return Err::<Assignment, JMUErr>(
                        JMUErr::new(
                            &format!(
                                "Received an empty string at position \"{:?}:{:?}\"!",
                                val_token.start.line,
                                val_token.start.column
                            )
                        )
                    )
                };
                value_buf.push(value);
            }
        }
        let key: String = match key_token.value{
            Some(key) => key,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        key_token.start.line,
                        key_token.start.column
                    )
                )
            )
        };
        Ok(Assignment::StringListAssign(key, value_buf))
    }

    /// A function to parse an assignment to a 
    /// list of boolean values. If the operation 
    /// is successful, the `BoolListAssign`
    /// variant of the `Assignment`
    /// enumeration is returned. If the
    /// operation fails, an error is
    /// returned.
    pub fn parse_bool_list_assign(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key_token: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let _open: Token = self.expect(&TokenType::OpenBracket)?;
        let mut value_buf: Vec<bool> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if self.is_done(){
                return Err::<Assignment, JMUErr>(
                    JMUErr::new("Unexpected end of token stream.")
                )
            }
            else if token.token_type == TokenType::CloseBracket{
                self.advance();
                break;
            }
            else {
                let val_token: Token;
                if self.peek_n(&1)?.token_type == TokenType::CloseBracket{
                    val_token = self.expect(&TokenType::Boolean)?;
                }
                else {
                    val_token = self.expect(&TokenType::Boolean)?;
                    let _comma: Token = self.expect(&TokenType::Comma)?;
                }
                let value: String = match val_token.value{
                    Some(value) => value,
                    None => return Err::<Assignment, JMUErr>(
                        JMUErr::new(
                            &format!(
                                "Received an empty string at position \"{:?}:{:?}\"!",
                                val_token.start.line,
                                val_token.start.column
                            )
                        )
                    )
                };
                if value == "true"{
                    value_buf.push(true);
                }
                else if value == "false"{
                    value_buf.push(false);
                }
                else {
                    return Err::<Assignment, JMUErr>(
                        JMUErr::new(
                            &format!(
                                "Received an unknown boolean string at position \"{:?}:{:?}\"!",
                                val_token.start.line,
                                val_token.start.column
                            )
                        )
                    )
                }
            }
        }
        let key: String = match key_token.value{
            Some(key) => key,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        key_token.start.line,
                        key_token.start.column
                    )
                )
            )
        };
        Ok(Assignment::BoolListAssign(key, value_buf))
    }

    /// A function to parse an assignment to a 
    /// list of integer values. If the operation 
    /// is successful, the `IntegerListAssign`
    /// variant of the `Assignment`
    /// enumeration is returned. If the
    /// operation fails, an error is
    /// returned.
    pub fn parse_integer_list_assign(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key_token: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let _open: Token = self.expect(&TokenType::OpenBracket)?;
        let mut value_buf: Vec<usize> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if self.is_done(){
                return Err::<Assignment, JMUErr>(
                    JMUErr::new("Unexpected end of token stream.")
                )
            }
            else if token.token_type == TokenType::CloseBracket{
                self.advance();
                break;
            }
            else {
                let val_token: Token;
                if self.peek_n(&1)?.token_type == TokenType::CloseBracket{
                    val_token = self.expect(&TokenType::Integer)?;
                }
                else {
                    val_token = self.expect(&TokenType::Integer)?;
                    let _comma: Token = self.expect(&TokenType::Comma)?;
                }
                let value: String = match val_token.value{
                    Some(value) => value,
                    None => return Err::<Assignment, JMUErr>(
                        JMUErr::new(
                            &format!(
                                "Received an empty string at position \"{:?}:{:?}\"!",
                                val_token.start.line,
                                val_token.start.column
                            )
                        )
                    )
                };
                let casted: usize = match value.parse::<usize>(){
                    Ok(casted) => casted,
                    Err(e) => return Err::<Assignment, JMUErr>(
                        JMUErr::new(&e.to_string())
                    )
                };
                value_buf.push(casted);
            }
        }
        let key: String = match key_token.value{
            Some(key) => key,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        key_token.start.line,
                        key_token.start.column
                    )
                )
            )
        };
        Ok(Assignment::IntegerListAssign(key, value_buf))
    }

    /// A function to parse an assignment to a 
    /// list of floating-point values. If the 
    /// operation is successful, the 
    /// `FloatingListAssign` variant 
    /// of the `Assignment` enumeration 
    /// is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_floating_point_list_assign(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key_token: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let _open: Token = self.expect(&TokenType::OpenBracket)?;
        let mut value_buf: Vec<f64> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if self.is_done(){
                return Err::<Assignment, JMUErr>(
                    JMUErr::new("Unexpected end of token stream.")
                )
            }
            else if token.token_type == TokenType::CloseBracket{
                self.advance();
                break;
            }
            else {
                let val_token: Token;
                if self.peek_n(&1)?.token_type == TokenType::CloseBracket{
                    val_token = self.expect(&TokenType::FloatingPoint)?;
                }
                else {
                    val_token = self.expect(&TokenType::FloatingPoint)?;
                    let _comma: Token = self.expect(&TokenType::Comma)?;
                }
                let value: String = match val_token.value{
                    Some(value) => value,
                    None => return Err::<Assignment, JMUErr>(
                        JMUErr::new(
                            &format!(
                                "Received an empty string at position \"{:?}:{:?}\"!",
                                val_token.start.line,
                                val_token.start.column
                            )
                        )
                    )
                };
                let casted: f64 = match value.parse::<f64>(){
                    Ok(casted) => casted,
                    Err(e) => return Err::<Assignment, JMUErr>(
                        JMUErr::new(&e.to_string())
                    )
                };
                value_buf.push(casted);
            }
        }
        let key: String = match key_token.value{
            Some(key) => key,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        key_token.start.line,
                        key_token.start.column
                    )
                )
            )
        };
        Ok(Assignment::FloatingListAssign(key, value_buf))
    }

    /// A function to parse a string
    /// assignment. If the operation is
    /// successful, the `StringAssign`
    /// variant of the `Assignment`
    /// enumeration is returned. If the
    /// operation fails, an error is
    /// returned.
    pub fn parse_string_assign(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key_token: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let value_token: Token = self.expect(&TokenType::UserString)?;
        let key: String = match key_token.value{
            Some(key) => key,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        key_token.start.line,
                        key_token.start.column
                    )
                )
            )
        };
        let value: String = match value_token.value{
            Some(value) => value,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        value_token.start.line,
                        value_token.start.column
                    )
                )
            )
        };
        Ok(Assignment::StringAssign(key, value))
    }

    /// A function to parse an integer
    /// assignment. If the operation is
    /// successful, the `IntegerAssign`
    /// variant of the `Assignment`
    /// enumeration is returned. If the
    /// operation fails, an error is
    /// returned.
    pub fn parse_integer_assign(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key_token: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let value_token: Token = self.expect(&TokenType::Integer)?;
        let key: String = match key_token.value{
            Some(key) => key,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        key_token.start.line,
                        key_token.start.column
                    )
                )
            )
        };
        let value: String = match value_token.value{
            Some(value) => value,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        value_token.start.line,
                        value_token.start.column
                    )
                )
            )
        };
        let number_cast: usize = match value.parse::<usize>(){
            Ok(number_cast) => number_cast,
            Err(e) => return Err::<Assignment, JMUErr>(
                JMUErr::new(&e.to_string())
            )
        };
        Ok(Assignment::IntegerAssign(key, number_cast))
    }

    /// A function to parse a floating-
    /// point assignment. If the operation
    /// is successful, the `FloatingAssign`
    /// variant of the `Assignment`
    /// enumeration is returned. If the
    /// operation fails, an error is
    /// returned.
    pub fn parse_floating_point_assign(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key_token: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let value_token: Token = self.expect(&TokenType::FloatingPoint)?;
        let key: String = match key_token.value{
            Some(key) => key,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        key_token.start.line,
                        key_token.start.column
                    )
                )
            )
        };
        let value: String = match value_token.value{
            Some(value) => value,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        value_token.start.line,
                        value_token.start.column
                    )
                )
            )
        };
        let number_cast: f64 = match value.parse::<f64>(){
            Ok(number_cast) => number_cast,
            Err(e) => return Err::<Assignment, JMUErr>(
                JMUErr::new(&e.to_string())
            )
        };
        Ok(Assignment::FloatingAssign(key, number_cast))
    }

    /// A function to parse a boolean
    /// assignment. If the operation is
    /// successful, the `BoolAssign`
    /// variant of the `Assignment`
    /// enumeration is returned. If the
    /// operation fails, an error is
    /// returned.
    pub fn parse_bool_assign(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key_token: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let value_token: Token = self.expect(&TokenType::Boolean)?;
        let key: String = match key_token.value{
            Some(key) => key,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        key_token.start.line,
                        key_token.start.column
                    )
                )
            )
        };
        let value: String = match value_token.value{
            Some(value) => value,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty string at position \"{:?}:{:?}\"!",
                        value_token.start.line,
                        value_token.start.column
                    )
                )
            )
        };
        let bool_value: bool;
        if value == "true"{
            bool_value = true;
        }
        else if value == "false"{
            bool_value = false;
        }
        else {
            return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an unknown boolean string at position \"{:?}:{:?}\"!",
                        value_token.start.line,
                        value_token.start.column
                    )
                )
            )
        }
        Ok(Assignment::BoolAssign(key, bool_value))
    }

    /// A function to parse a nested
    /// assignment. If the operation is
    /// successful, the `NestedAssign`
    /// variant of the `Assignment`
    /// enumeration is returned. If the
    /// operation fails, an error is
    /// returned.
    pub fn parse_nested_assignment(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key_token: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let _marker: Token = self.expect(&TokenType::SectionMarker)?;
        let mut inner_assignments: Vec<Assignment> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if token.token_type == TokenType::SectionMarker{
                self.advance();
                break;
            }
            else if self.is_done(){
                return Err::<Assignment, JMUErr>(
                    JMUErr::new("Missing nested-section delimiter.")
                );
            }
            else {
                inner_assignments.push(self.parse_assignment()?);
            }
        }
        let key: String = match key_token.value{
            Some(key) => key,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty key at position \"{:?}:{:?}\"!",
                        key_token.start.line,
                        key_token.start.column
                    )
                )
            )
        };
        Ok(Assignment::NestedAssign(key, Box::new(inner_assignments)))
    }

    /// A function to parse an assignment to a 
    /// list of further assignments. If the 
    /// operation is successful, the 
    /// `AssignmentList` variant 
    /// of the `Assignment` enumeration 
    /// is returned. If the operation fails, 
    /// an error is returned.
    pub fn parse_assignment_list(
        &mut self
    ) -> Result<Assignment, JMUErr>{
        let key: Token = self.expect(&TokenType::UserString)?;
        let _assign: Token = self.expect(&TokenType::Assign)?;
        let _open_bracket: Token = self.expect(&TokenType::OpenBracket)?;
        let mut assignment_blocks: Vec<Box<Vec<Assignment>>> = Vec::new();
        while let Some(token) = self.stream.get(self.cursor){
            if self.stream.len() == self.cursor {
                return Err::<Assignment, JMUErr>(
                    JMUErr::new("Unclosed delimiter.")
                );
            }
            else if token.token_type == TokenType::CloseBracket{
                self.advance();
                break;
            }
            else {
                let _open_delim: Token = self.expect(&TokenType::SectionMarker)?;
                let mut assignments: Vec<Assignment> = Vec::new();
                while let Some(token) = self.stream.get(self.cursor){
                    if self.is_done(){
                        return Err::<Assignment, JMUErr>(
                            JMUErr::new("Unexpected end of token stream.")
                        )
                    }
                    else if token.token_type == TokenType::SectionMarker{
                        if self.peek_n(&1)?.token_type == TokenType::CloseBracket{
                            self.advance();
                            break;
                        }
                        else {
                            self.advance();
                            let _comma: Token = self.expect(&TokenType::Comma)?;
                            break;
                        }
                    }
                    else {
                        assignments.push(self.parse_assignment()?);
                    }
                }
                assignment_blocks.push(Box::new(assignments));
            }
        }
        let key_str: String = match key.value{
            Some(key_str) => key_str,
            None => return Err::<Assignment, JMUErr>(
                JMUErr::new(
                    &format!(
                        "Received an empty key at position \"{:?}:{:?}\"!",
                        key.start.line,
                        key.start.column
                    )
                )
            )
        };
        Ok(Assignment::AssignmentList(key_str, assignment_blocks))
    }
}
