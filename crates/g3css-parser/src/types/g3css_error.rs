use crate::Rule;
use pest::error::Error;

// Define a custom error enum for G3css-related errors
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssError {
    // Represents errors that occur during parsing
    ParseError(Error<Rule>),
    // Represents other types of errors with a custom error message
    OtherError(String),
}
