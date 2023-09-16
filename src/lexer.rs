mod token;

use std::iter::{Enumerate, Peekable};

pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    source: Peekable<Enumerate<I>>,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(source: I) -> Self {
        Self {
            source: source.enumerate().peekable(),
        }
    }
}