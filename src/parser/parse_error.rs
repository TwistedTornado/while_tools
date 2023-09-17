use crate::lexer::Span;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents an error in the parsing stage. Use this when you want to signify
/// that, during parsing, you found an unexpected token or encountered some
/// error with the token stream.
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ParseError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}
