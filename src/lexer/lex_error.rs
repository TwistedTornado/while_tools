use crate::lexer::span::Span;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents an error in the lexing stage. Use this when you want to signify
/// that, during lexing, you found an unknown character or encountered some
/// error with the character stream.
#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

impl LexError {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lexing error: {} (At {}-{})",
            self.message, self.span.0, self.span.1
        )
    }
}

impl Error for LexError {}
