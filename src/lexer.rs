mod lex_error;
mod span;
mod token;

use crate::lexer::token::Token::*;
use std::iter::Peekable;

pub use crate::lexer::lex_error::LexError;
pub use crate::lexer::span::{Span, Spanned};
pub use crate::lexer::token::Token;

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
        self.current_index += 1;
        self.source.next()
    }

    fn eat_ident(&mut self, start: char) -> Token {
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
            '!' => Not,

            '<' => {
                if let Some(&'=') = self.peek() {
                    self.advance();
                    LessEqual
                } else {
                    Unknown
                }
            }

            ':' => {
                if let Some(&'=') = self.peek() {
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
