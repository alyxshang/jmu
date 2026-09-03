/*
JMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Deserialize"
/// trait to use on structures
/// for testing deserialization.
use serde::Deserialize;

/// Importing the "Token" 
/// structure for explicit
/// typing.
use super::lexer::Token;

/// Importing the "Parser"
/// structure to test it.
use super::parser::Parser;

/// Importing the function
/// to tokenize JMU source
/// code to test it.
use super::lexer::tokenize;

/// Importing the enumeration
/// that builds the foundation
/// of the JMU AST for explicit
/// typing.
use super::parser::Assignment;

/// Importing the function for
/// deserializing JMU source code
/// into a Rust data structure.
use super::deserialize::from_str;

/// Testing the function
/// to tokenize JMU's code.
#[test]
pub fn test_lexer(){
    let slice = r#"
        "name" >~< "Alyx"
        "age" >~< 26
        "is_awesome" >~< true
        "my_hobbies" >~< ["code", "art", "music"]
        "truth_table" >~<  [true, false, true]
    "#;
    let tokenized: Vec<Token> = tokenize(&slice.to_string())
        .expect("Could not tokenize string");
    assert_eq!(tokenized.len(), 27);
}

/// Testing the structure
/// containing functions 
/// to parse JMU's code.
#[test]
pub fn test_parser(){
    let slice = r#"
        "name" >~< "Alyx"
        "age" >~< 2026
        "is_awesome" >~< true
        "is_political" >~< false
        "hobbies" >~< ["code", "art", "music"]
        "truth_table" >~< [true, false, true]
        # This is a comment.
        "nested_testing" >~< |
            "one" >~< "two"
            # Another comment!
            "three" >~< "four"
            "five" >~< "six"
            "pi" >~< 3.1415
        |
        "continuation" >~< true
        "nums" >~< [1,2,3,4,5]
        "my_nums" >~< [1.2, 2.3, 3.4]
    "#;
    let tokenized: Vec<Token> = tokenize(&slice.to_string())
        .expect("Could not tokenize string");
    let mut parser: Parser = Parser::new(&tokenized)
        .expect("Could not construct parser from token stream.");
    let parsed: Vec<Assignment> = parser.parse()
        .expect("Parsing failure.");
    let expected: Vec<Assignment> = vec![
        Assignment::StringAssign(
            "name".to_string(), 
            "Alyx".to_string()
        ), 
        Assignment::IntegerAssign(
            "age".to_string(), 
            2026
        ), 
        Assignment::BoolAssign(
            "is_awesome".to_string(), 
            true
        ), 
        Assignment::BoolAssign(
            "is_political".to_string(), 
            false
        ), 
        Assignment::StringListAssign(
            "hobbies".to_string(), 
            vec![
                "code".to_string(), 
                "art".to_string(), 
                "music".to_string()
            ]
        ), 
        Assignment::BoolListAssign(
            "truth_table".to_string(), 
            vec![
                true, 
                false, 
                true
            ]
        ),
        Assignment::Comment,
        Assignment::NestedAssign(
            "nested_testing".to_string(), 
            Box::new(
                vec![
                    Assignment::StringAssign(
                        "one".to_string(), 
                        "two".to_string()
                    ),
                    Assignment::Comment,
                    Assignment::StringAssign(
                        "three".to_string(), 
                        "four".to_string()
                    ), 
                    Assignment::StringAssign(
                        "five".to_string(), 
                        "six".to_string()
                    ),
                    Assignment::FloatingAssign(
                        "pi".to_string(), 
                        3.1415
                    )
                ]
            )
        ),
        Assignment::BoolAssign(
            "continuation".to_string(),
            true
        ),
        Assignment::IntegerListAssign(
            "nums".to_string(), 
            vec![1,2,3,4,5]
        ),
        Assignment::FloatingListAssign(
            "my_nums".to_string(), 
            vec![1.2, 2.3, 3.4]
        ),
    ]; 
    assert_eq!(parsed, expected);
}


/// A structure containing
/// strongly-typed data 
/// to model the structure
/// of a JMU document.
#[derive(Deserialize, Debug)]
pub struct TestData{
    pub pi: f64,
    pub age: usize,
    pub name: String,
    pub is_awesome: bool,
    pub is_political: bool,
    pub continuation: bool,
    pub hobbies: Vec<String>,
    pub truth_table: Vec<bool>,
    pub nested_testing: NestedTesting,
    pub optional_data: Option<Vec<String>>,
}

/// A structure containing
/// strongly-typed data to
/// model a nested section
/// of a JMU document.
#[derive(Deserialize, Debug)]
pub struct NestedTesting {
    pub one: String,
    pub three: String,
    pub five: String,
    pub six: Vec<usize>,
    pub seven: Vec<f64>,
    pub eight: Option<bool>
}

/// A function to test the
/// `from_str` function with
/// an optional field present
/// in the markup.
#[test]
pub fn test_deserializer_with_some(){
    let slice = r#"
        "name" >~< "Alyx"
        "age" >~< 2026
        "pi" >~< 3.1415
        "is_awesome" >~< true
        "is_political" >~< false
        "hobbies" >~< ["code", "art", "music"]
        "truth_table" >~< [true, false, true]
        "optional_data" >~< ["foo", "bar", "baz"]
        # This is a comment.
        "nested_testing" >~< |
            "one" >~< "two"
            # Another comment!
            "three" >~< "four"
            "five" >~< "six"
            "six" >~< [1,2,3,4]
            "seven" >~< [1.2, 2.3, 3.4]
            "eight" >~< true
        |
        "continuation" >~< true
    "#;
    let deserialized: TestData = from_str(&slice.to_string())
        .expect("Could not deserialize data.");
    assert_eq!(deserialized.name, "Alyx".to_string());
    assert_eq!(deserialized.optional_data.is_some(), true);

}

/// A function to test the
/// `from_str` function with
/// an optional field not present
/// in the markup.
#[test]
pub fn test_deserializer_with_none(){
    let slice = r#"
        "name" >~< "Alyx"
        "age" >~< 2026
        "pi" >~< 3.1415
        "is_awesome" >~< true
        "is_political" >~< false
        "hobbies" >~< ["code", "art", "music"]
        "truth_table" >~< [true, false, true]
        # This is a comment.
        "nested_testing" >~< |
            "one" >~< "two"
            # Another comment!
            "three" >~< "four"
            "five" >~< "six"
            "six" >~< [1,2,3,4]
            "seven" >~< [1.2, 2.3, 3.4]
        |
        "continuation" >~< true
    "#;
    let deserialized: TestData = from_str(&slice.to_string())
        .expect("Could not deserialize data.");
    assert_eq!(deserialized.name, "Alyx".to_string());
    assert_eq!(deserialized.optional_data.is_some(), false);
}

/// A data structure to hold the
/// deserialized data about
/// a character list for testing
/// deserialization of lists of
/// assignments.
#[derive(Deserialize, Debug)]
pub struct CharacterList{
    pub id: String,
    pub characters: String,
    pub nested_list: Vec<Character>
}

/// A data structure to hold the
/// deserialized data about
/// a character list for testing
/// deserialization of lists of
/// assignments.
#[derive(Deserialize, Debug)]
pub struct Character {
    pub name: String,
    pub age: usize
}

/// Testing deserialization of assignment
/// lists.
#[test]
pub fn test_assignment_list(){
    let slice: &str = r#"
        "id" >~< "People"
        "characters" >~< "Divine Beings"
        "nested_list" >~< [
            |
                "name" >~< "Alyx"
                "age" >~< 2025
            |,
            |
                "name" >~< "Zibai"
                "age" >~< 6000
            |,
            |
                "name" >~< "Xianyun"
                "age" >~< 4000
            |

        ]
    "#;
    let deserialized: CharacterList = from_str(&slice.to_string())
        .expect("Could not deserialize data.");
    println!("{:?}", deserialized);
    assert_eq!(deserialized.nested_list.len(), 3);
}
