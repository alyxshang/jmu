/*
JMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// to catch and handle errors.
use super::err::JMUErr;

/// A function to check
/// whether a character
/// is numeric or not.
/// If the passed `Option<char>`
/// is empty, an error is returned.
/// A boolean is returned
/// depending on whether the
/// character is numeric or not.
pub fn is_numeric(
    sub: &Option<char>
) -> Result<bool, JMUErr> {
    let chars: Vec<char> = "1234567890"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let character: char = match sub{
        Some(character) => *character,
        None => return Err::<bool, JMUErr>(
            JMUErr::new("Here")
        )
    };
    Ok(chars.contains(&character))
}

/// A structure to capture the 
/// current position of a character
/// and provide information on the 
/// start and end of a JMU token.
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize
}

/// An enumeration "listing"
/// all the possible types of 
/// tokens that can exist in 
/// JMU markup.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Comma,
    Assign,
    Boolean,
    Integer,
    UserString,
    UserComment,
    OpenBracket,
    CloseBracket,
    SectionMarker,
    FloatingPoint
}

/// A structure to encapsulate
/// information on a captured
/// token.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub end: Position,
    pub start: Position,
    pub value: Option<String>,
    pub token_type: TokenType
}

/// A function to tokenize
/// JMU source code. If the 
/// operation is successful,
/// a vector of instances of
/// the `Token` structure is
/// returned. If the operation
/// fails, an error is returned.
pub fn tokenize(
    sub: &str
) -> Result<Vec<Token>, JMUErr>{
    let chars: Vec<char> = sub
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    if chars.is_empty(){
        Err::<Vec<Token>, JMUErr>(
            JMUErr::new("The supplied string cannot be empty.")
        )
    }
    else{
        let mut line: usize = 0;
        let mut column: usize = 0;
        let mut cursor: usize = 0;
        let mut stream: Vec<Token> = Vec::new();
        while cursor < chars.len(){
            if chars.get(cursor) == Some(&'"'){
                cursor += 1;
                let column_start: usize = column;
                let mut str_buf: Vec<char> = Vec::new();
                while chars.get(cursor) != Some(&'"'){
                    let character: char = match chars.get(cursor){
                        Some(character) => *character,
                        None => return Err::<Vec<Token>, JMUErr>(
                            JMUErr::new("Unexpected end of token stream!")
                        )
                    };
                    str_buf.push(character);
                    cursor += 1;
                    column += 1;
                }
                stream.push(
                    Token{
                        end: Position{ line: line, column: column }, 
                        start: Position{ line: line, column: column_start },
                        value: Some(str_buf.iter().collect::<String>()),
                        token_type: TokenType::UserString
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'>') &&
                chars.get(cursor + 1) == Some(&'~') &&
                chars.get(cursor + 2) == Some(&'<')
            {
                stream.push(
                    Token{
                        end: Position{ line: line, column: column + 2 }, 
                        start: Position{ line: line, column: column }, 
                        value: None,
                        token_type: TokenType::Assign
                    }
                );
                column += 3;
                cursor += 3;
            }
            else if chars.get(cursor) == Some(&','){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column },
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::Comma
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'t') &&
                chars.get(cursor + 1) == Some(&'r') &&
                chars.get(cursor + 2) == Some(&'u') &&
                chars.get(cursor + 3) == Some(&'e')
            {
                stream.push(
                    Token{
                        end: Position{ line: line, column: column + 3 }, 
                        start: Position{ line: line, column: column },
                        value: Some("true".to_string()),
                        token_type: TokenType::Boolean
                    }
                );
                column += 4;
                cursor += 4;
            }
            else if chars.get(cursor) == Some(&'f') &&
                chars.get(cursor + 1) == Some(&'a') &&
                chars.get(cursor + 2) == Some(&'l') &&
                chars.get(cursor + 3) == Some(&'s') &&
                chars.get(cursor + 4) == Some(&'e')
            {
                stream.push(
                    Token{
                        end: Position{ line: line, column: column + 4 }, 
                        start: Position{ line: line, column: column },
                        value: Some("false".to_string()),
                        token_type: TokenType::Boolean
                    }
                );
                column += 5;
                cursor += 5;
            }
            else if chars.get(cursor) == Some(&'['){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column }, 
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::OpenBracket
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&']'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column }, 
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::CloseBracket
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'\n') ||
                chars.get(cursor) == Some(&'\r')
            {
                line += 1;
                column = 0;
                cursor += 1;
            } 
            else if chars.get(cursor) == Some(&'\t') ||
                chars.get(cursor) == Some(&' ')
            {
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'|'){
                stream.push(
                    Token{
                        end: Position{ line: line, column: column }, 
                        start: Position{ line: line, column: column },
                        value: None,
                        token_type: TokenType::SectionMarker
                    }
                );
                column += 1;
                cursor += 1;
            }
            else if chars.get(cursor) == Some(&'\r') &&
                chars.get(cursor + 1) == Some(&'\n')
            {
                line += 1;
                column = 0;
                cursor += 2;
            }
            else if chars.get(cursor) == Some(&'#'){
                let column_start: usize = column;
                cursor += 1;
                while let Some(character) = chars.get(cursor){
                    if character == &'\n' || character == &'\r'{
                        break;
                    }
                    else {
                        cursor += 1;
                        column += 1;
                    }
                }
                stream.push(
                    Token{
                        end: Position{ line: line, column: column }, 
                        start: Position{ line: line, column: column_start },
                        value: None,
                        token_type: TokenType::UserComment
                    }
                );
            }
            else if is_numeric(&chars.get(cursor).cloned())?{
                let column_start: usize = column;
                let mut num_buf: Vec<String> = Vec::new();
                let mut im_buffer: Vec<char> = Vec::new();
                while let Some(character) = chars.get(cursor){ 
                    if is_numeric(&chars.get(cursor).cloned())? ||
                       character == &'.'
                    { 
                        if character == &'.'{
                            num_buf.push(im_buffer.iter().collect::<String>());
                            im_buffer.clear();
                        }
                        else {
                            im_buffer.push(*character);
                        }
                        cursor += 1;
                        column += 1;
                    }
                    else {
                        break;
                    }
                }
                num_buf.push(im_buffer.iter().collect::<String>());
                if num_buf.len() == 2{
                    let front: String = num_buf[0].clone();
                    let back: String = num_buf[1].clone();
                    let number: String = format!("{}.{}", front, back);
                    stream.push(
                        Token{
                            end: Position{ line: line, column: column }, 
                            start: Position{ line: line, column: column_start },
                            value: Some(number),
                            token_type: TokenType::FloatingPoint
                        }
                    );
                }
                else if num_buf.len() == 1{
                    let number: String = num_buf[0].clone();
                    stream.push(
                        Token{
                            end: Position{ line: line, column: column }, 
                            start: Position{ line: line, column: column_start },
                            value: Some(number),
                            token_type: TokenType::Integer
                        }
                    );
                }
                else {
                    let e: String = format!(
                        "Unexepected character at position \"{}\"!",
                        cursor
                    );
                    return Err::<Vec<Token>, JMUErr>(
                        JMUErr::new(&e.to_string())
                    );
                }
            }
            else{
                let e: String = format!(
                    "Unexepected character at position \"{:?}\"!",
                    cursor
                );
                return Err::<Vec<Token>, JMUErr>(
                    JMUErr::new(&e.to_string())
                );
            }
        }
        Ok(stream)
    }
}
