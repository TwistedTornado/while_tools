use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents an error in the interpreting stage. Use this when you want to signify
/// that, during interpreting, you found an undefined name or a bad type.
#[derive(Debug)]
pub struct InterpretError(pub String);

impl Error for InterpretError {}

impl Display for InterpretError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
