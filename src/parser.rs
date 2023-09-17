use crate::lexer::{Spanned, Token};
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
}
