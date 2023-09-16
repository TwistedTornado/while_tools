mod lex_error;
mod span;
mod token;

use crate::lexer::lex_error::LexError;
use crate::lexer::span::{Span, Spanned};
use crate::lexer::token::Token::{self, *};
use std::iter::Peekable;

pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    source: Peekable<I>,
    current_index: usize,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new<II: IntoIterator<Item = char, IntoIter = I>>(source: I) -> Self {
        Self {
            source: source.into_iter().peekable(),
            current_index: 0,
        }
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn advance(&mut self) -> Option<char> {
        self.current_index += 1;
        self.source.next()
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Spanned<Token>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
