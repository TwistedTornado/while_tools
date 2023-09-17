mod parse_error;

use crate::lexer::{Span, Spanned, Token};
use crate::parser::parse_error::ParseError;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Parser<'a, I>
where
    I: Iterator<Item = Spanned<Token>>,
{
    source: &'a str,
    tokens: Peekable<I>,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Spanned<Token>>,
{
    pub fn new<II: IntoIterator<Item = Spanned<Token>, IntoIter = I>>(
        source: &'a str,
        tokens: II,
    ) -> Self {
        Self {
            source,
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Returns the next Spanned<Token>, and advances the stream in doing so.
    /// When the token stream is finished, returns `None`.
    /// Skips `Whitespace` and `LineBreak` tokens.
    fn advance(&mut self) -> Option<Spanned<Token>> {
        let next_token = self.tokens.next();

        if matches!(
            next_token,
            Some(Spanned {
                inner: Token::Whitespace | Token::LineBreak,
                ..
            })
        ) {
            self.advance()
        } else {
            next_token
        }
    }

    fn peek(&mut self) -> Option<&Spanned<Token>> {
        self.tokens.peek()
    }

    /// Checks that the next token is as specified. If it is, the token is
    /// consumed and the stream advances. If not, return a ParseError.
    fn expect_token(&mut self, token: Token) -> Result<(), ParseError> {
        let retrieved = self.advance();

        match retrieved {
            Some(Spanned { inner, .. }) if inner == token => Ok(()),

            Some(Spanned { inner, span }) => Err(ParseError {
                message: format!("Expected {token:?}, found {inner:?}",),
                span,
            }),

            _ => Err(ParseError {
                message: format!("Expected {token:?}, but reached end of token stream",),
                span: Span(self.source.len(), self.source.len() + 1),
            }),
        }
    }

    /// Optionally expects a token. If the token exists, acts like `expect_token`.
    /// Otherwise, does nothing, not even returning an error or panic!()-ing.
    fn maybe_expect_token(&mut self, token: Token) {
        match self.peek() {
            Some(ti) if ti.inner == token => {
                self.advance();
            }
            _ => {}
        };
    }
}
