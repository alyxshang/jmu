/*
JMU by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// namespace of
/// "serde::de".
use serde::de::{self};

/// Importing the structure
/// to catch and handle errors.
use super::err::JMUErr;

/// Importing the "IntoIter"
/// trait.
use std::vec::IntoIter;

/// Importing the entity
/// that traverses data
/// to deserialize it.
use serde::de::Visitor;

/// Importing the structure
/// that encapsulates data
/// about a captured JMU
/// token.
use super::lexer::Token;

/// Importing another
/// trait to implement the
/// "Deserialize" trait for
/// JMU.
use serde::de::MapAccess;

/// Importing another
/// trait to implement
/// the "Deserialize" trait
/// for JMU.
use serde::de::SeqAccess;

/// Importing the structure
/// containing functions and
/// data to build an AST from
/// a stream of JMU tokens.
use super::parser::Parser;

/// Importing the function to
/// tokenize a string of JMU
/// source code.
use super::lexer::tokenize;

/// Importing the enumeration
/// that builds the foundation
/// of the JMU AST for explicit
/// typing.
use super::parser::Assignment;

/// Importing a trait from the
/// "serde" crate to implement
/// the "Deserialize" trait.
use serde::de::DeserializeSeed;

/// Importing a trait from the
/// "serde" crate to implement
/// the "Deserialize" trait.
use serde::de::IntoDeserializer;

/// Importing the `DeserializeOwned`
/// trait to use it in a function
/// definition for constraining the
/// allowed data type.
use serde::de::DeserializeOwned;

/// A structure to encapsulate data
/// about the process of deserializing
/// the AST parsed from JMU markup.
pub struct AstDeserializer {
    iter: IntoIter<Assignment>,
    current_value: Option<Assignment>,
}


/// Implementing function(s)
/// for the `AstDeserializer`
/// structure.
impl AstDeserializer {

    /// A function to create a new instance of the
    /// `AstDeserializer` structure and return it.
    /// If the operation fails, an error is returned.
    pub fn new(
        assignments: Vec<Assignment>
    ) -> Result<AstDeserializer, JMUErr> {
        if assignments.is_empty() {
            Err::<AstDeserializer, JMUErr>(
                JMUErr::new("AST cannot be empty.")
            )
        }
        else{
            Ok(
                AstDeserializer {
                    iter: assignments.into_iter(),
                    current_value: None,
                }
            )
        }
    }
}

/// Implementing the `Deserialize` trait to 
/// "explain" to serde how to handle the top-level
/// object of the AST.
impl<'de> de::Deserializer<'de> for &mut AstDeserializer {

    /// Declaring the error type
    /// to catch and handle errors.
    type Error = JMUErr;

    /// Traversing each type of node in
    /// the AST and returning an error
    /// if the operation fails. This
    /// also specifically handles options.
    fn deserialize_option<V: Visitor<'de>>(
        self, 
        visitor: V
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_some(self)
    }

    /// Traversing each type of node in the
    /// AST and returning an error if the
    /// operation fails. The `MapAccess`
    /// trait is used here.
    fn deserialize_any<V: Visitor<'de>>(
        self, 
        visitor: V
    ) -> Result<V::Value, Self::Error>{
        visitor.visit_map(self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

/// Implementing the `MapAccess`
/// trait to treat each assignment pair
/// as a key-value pair.
impl<'de> MapAccess<'de> for AstDeserializer {

    /// Declaring the error type
    /// to catch and handle errors.
    type Error = JMUErr;

    /// A function to obtain the key for a given 
    /// assignment. If the operation fails, an 
    /// error is returned.
    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self, 
        seed: K
    ) -> Result<Option<K::Value>, Self::Error>{
        for assignment in self.iter.by_ref(){          
            match assignment {
                Assignment::Comment => continue, // Skip comments
                _ => {
                    let key = match &assignment {
                        Assignment::BoolAssign(k, _) |
                        Assignment::StringAssign(k, _) |
                        Assignment::BoolListAssign(k, _) |
                        Assignment::StringListAssign(k, _) |
                        Assignment::FloatingAssign(k, _) |
                        Assignment::IntegerAssign(k, _) |
                        Assignment::IntegerListAssign(k, _) |
                        Assignment::FloatingListAssign(k, _) |
                        Assignment::AssignmentList(k, _) |
                        Assignment::NestedAssign(k, _) => k.clone(),
                        _ => unreachable!(),
                    };
                    self.current_value = Some(assignment);
                    return seed.deserialize(key.into_deserializer()).map(Some);
                }
            }
        }
        Ok(None)
    }

    /// A function to obtain the value for a given key.
    /// If the operation fails, an error is returned.
    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self, 
        seed: V
    ) -> Result<V::Value, Self::Error> {
        let assignment = self
            .current_value
            .take()
            .ok_or_else(
                || JMUErr::new("Value accessed without key")
            )?;
        seed.deserialize(ValueDeserializer(assignment))
    }
}

/// Implementing the `SeqAccess`
/// trait to handle nested assignment
/// lists.
impl<'de> SeqAccess<'de> for AssignmentListAccess {

    /// Declaring the error type
    /// to catch and handle errors.
    type Error = JMUErr;

    /// A function to iterate over the list of
    /// items in a list of assignments.
    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self, seed: T
    ) -> Result<Option<T::Value>, Self::Error>{
        match self.iter.next() {
            Some(assignments) => {
                let mut de = AstDeserializer::new(*assignments)?;
                seed.deserialize(&mut de).map(Some)
            }
            None => Ok(None),
        }
    }
}

/// A structure to deserialize
/// a single instance of the 
/// `Assignment` enumeration.
struct ValueDeserializer(Assignment);

/// A structure to deserialize
/// a list of instances of the 
/// `Assignment` enumeration.
struct AssignmentListAccess {
    iter: IntoIter<Box<Vec<Assignment>>>
}

/// Implementing the `Deserialize` trait for the
/// `ValueDeserializer` structure.
impl<'de> de::Deserializer<'de> for ValueDeserializer {

    /// Declaring the error type
    /// to catch and handle errors.
    type Error = JMUErr;

    /// A function to deserialize every
    /// single possible variant of the
    /// `Assignment` enumeration.
    fn deserialize_any<V: Visitor<'de>>(
        self, 
        visitor: V
    ) -> Result<V::Value, Self::Error> {
        match self.0 {
            Assignment::BoolAssign(_, v) => 
                visitor.visit_bool(v),
            Assignment::StringAssign(_, v) => 
                visitor.visit_string(v),
            Assignment::IntegerAssign(_, v) => 
                visitor.visit_i64(v as i64),
            Assignment::FloatingAssign(_, v) => 
                visitor.visit_f64(v),
            Assignment::BoolListAssign(_, v) => 
                visitor.visit_seq(v.into_deserializer()),
            Assignment::StringListAssign(_, v) => 
                visitor.visit_seq(v.into_deserializer()),
            Assignment::IntegerListAssign(_, v) => 
                visitor.visit_seq(v.into_deserializer()),
            Assignment::FloatingListAssign(_, v) => 
                visitor.visit_seq(v.into_deserializer()),
            Assignment::NestedAssign(_, inner) => {
                let mut de = AstDeserializer::new(*inner)?;
                visitor.visit_map(&mut de)
            },
            Assignment::AssignmentList(_, inner) => {
               visitor.visit_seq(
                   AssignmentListAccess {
                    iter: inner.into_iter(),
                    }
                ) 
            },
            Assignment::Comment => unreachable!(),
        }
    }

    /// A function to specifically 
    /// handle the `Option<T>` type 
    /// in a deserialization.
    fn deserialize_option<V: Visitor<'de>>(
        self, 
        visitor: V
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_some(self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

/// A function to deserialize
/// a string of JMU source code
/// into the data structure of type
/// `T` if `T` implements the 
/// `Deserialize` trait.
/// If the operation is successful,
/// an instance of `T` is returned.
/// If the operation fails, an error
/// is returned.
pub fn from_str<T: DeserializeOwned>(
    sub: &str
) -> Result<T, JMUErr>{
    let tokenized: Vec<Token> = tokenize(sub)?;
    let mut parser: Parser = Parser::new(&tokenized)?;
    let parsed: Vec<Assignment> = parser.parse()?;
    let mut deserializer: AstDeserializer = AstDeserializer::new(
        parsed
    )?;
    T::deserialize(&mut deserializer)
}
