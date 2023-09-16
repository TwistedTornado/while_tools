use crate::lexer::span::Span;
use std::error::Error;
use std::fmt::{Display, Formatter};

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