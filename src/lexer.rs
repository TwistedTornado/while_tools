//! A module for lexing tools for While.
//!
//! This module provides a struct [`Lexer`] that takes an iterator over
//! characters and is, itself, an [`Iterator`] that returns
//! `Result<Spanned<Token>, ParseError>`.
//!
//! This module also provides various utility structs such as some`Span` types
//! and a [`LexError`] type.
mod lex_error;
mod span;
mod token;

use crate::lexer::token::Token::*;
use std::iter::Peekable;

pub use crate::lexer::lex_error::LexError;
pub use crate::lexer::span::{Span, Spanned};
pub use crate::lexer::token::Token;

/// Lexes an incoming character stream. The resulting iterator contains no
/// concrete information about literals or identifiers -- this can be accessed
/// by using the `Span`s provided.
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
    pub fn new<II: IntoIterator<Item = char, IntoIter = I>>(source: II) -> Self {
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
        // Storing our own `current_index` rather than using an [`Enumerate`] is
        // just a simpler and cleaner solution at the moment.
        self.current_index += 1;
        self.source.next()
    }

    fn eat_ident(&mut self, start: char) -> Token {
        // Since we can't backtrack, we pass the recently-advanced character to
        // this function. Unlike the other `eat_*` functions, it requires
        // knowledge of all the characters in order to work out what Token
        // to return.

        let mut ident_buffer = String::from(start);

        while self.peek().is_some_and(|c| c.is_alphanumeric()) {
            ident_buffer.push(self.advance().unwrap());
        }

        match ident_buffer.as_str() {
            "if" => If,
            "then" => Then,
            "else" => Else,
            "while" => While,
            "do" => Do,
            "skip" => Skip,
            "true" => True,
            "false" => False,
            _ => Identifier,
        }
    }

    fn eat_whitespaces(&mut self) -> Token {
        while let Some(' ' | '\t') = self.peek() {
            self.advance();
        }
        Whitespace
    }

    fn eat_numbers(&mut self) -> Token {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance().unwrap();
        }
        Literal
    }

    fn eat_linebreaks(&mut self) -> Token {
        while let Some('\n' | '\r') = self.peek() {
            self.advance();
        }
        Semicolon
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Spanned<Token>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.current_index;

        let c = match self.advance() {
            None => return None,
            Some(t) => t,
        };

        let token = match c {
            '(' => LeftParen,
            ')' => RightParen,

            '+' => Add,
            '-' => Subtract,
            '*' => Multiply,

            '=' => Equal,
            '&' => And,

            '[' => match self.peek() {
                Some('[') => {
                    self.advance();
                    LeftSemantic
                }
                _ => Unknown,
            },

            ']' => match self.peek() {
                Some(']') => {
                    self.advance();
                    RightSemantic
                }
                _ => Unknown,
            },

            '!' => match self.peek() {
                Some('=') => {
                    self.advance();
                    NotEqual
                }
                _ => Not,
            },

            '<' => match self.peek() {
                Some('=') => {
                    self.advance();
                    LessEqual
                }
                _ => LessThan,
            },

            '>' => match self.peek() {
                Some('=') => {
                    self.advance();
                    GreaterEqual
                }
                _ => GreaterThan,
            },

            ':' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Assign
                } else {
                    Unknown
                }
            }

            '0'..='9' => self.eat_numbers(),

            'a'..='z' | 'A'..='Z' => self.eat_ident(c),

            ' ' | '\t' => self.eat_whitespaces(),

            '\r' | '\n' | ';' => self.eat_linebreaks(),

            _ => Unknown,
        };

        let span = Span(start, self.current_index);

        let output = match token {
            Unknown => Err(LexError::new("Unknown token".to_string(), span)),
            t => Ok(Spanned::new(t, span)),
        };

        Some(output)
    }
}
