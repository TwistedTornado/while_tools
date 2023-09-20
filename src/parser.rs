mod parse_error;

use crate::ast::Ast;
use crate::lexer::{Span, Spanned, Token};
use crate::parser::parse_error::ParseError;
use crate::{
    add, and, ass_stmt, binary_node, comp_stmt, eq, if_stmt, less_eq, literal, mul, not, sub,
    while_stmt,
};

use std::iter::Peekable;

/// A parser that transforms a stream of tokens into an AST.
/// Needs a reference to the source, to be able to extract identifiers and
/// literal values.
///
/// The parser uses recursive descent, and distinguishes between statements
/// and expressions.
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
        let mut next_token = self.tokens.next();

        while next_token
            .is_some_and(|spanned| matches!(spanned.inner, Token::Whitespace | Token::LineBreak))
        {
            next_token = self.tokens.next();
        }

        next_token
    }

    /// Returns a reference to the next Spanned<Token>, and does _not_ advance
    /// the stream in doing so.
    ///
    /// When the token stream is finished, returns `None`.
    /// Skips `Whitespace` and `LineBreak` tokens.
    fn peek(&mut self) -> Option<&Spanned<Token>> {
        while self
            .tokens
            .peek()
            .is_some_and(|spanned| matches!(spanned.inner, Token::Whitespace | Token::LineBreak))
        {
            self.tokens.next();
        }

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
                span: Span(self.source.len() - 1, self.source.len()),
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

    // Parse the token stream into an abstract syntax tree representing the
    // structure of the program. Currently, this is span-less.
    pub fn parse(&mut self) -> Result<Ast, ParseError> {
        self.stmt_block()
    }

    fn stmt_block(&mut self) -> Result<Ast, ParseError> {
        // <stmt_block> ::= <statement> (";" <statement>)* ";"?

        // This is temporary, to allow multiline source files.
        // Internally, newlines are condensed to semicolons more liberally than
        // suitable, so the ASI logic needs to be expanded upon.
        while self
            .peek()
            .is_some_and(|Spanned { inner, .. }| inner == &Token::Semicolon)
        {
            self.advance();
        }

        let mut stmt = self.statement()?;

        // If the next token is a semicolon, remove it and find the next <statement>.
        while let Some(Spanned {
            inner: Token::Semicolon,
            ..
        }) = self.peek()
        {
            self.advance();

            // These tokens indicate an end of the statement block, and so we
            // now pass control back to the function that called them.
            // For example, maybe we reached here after advancing the semicolon:
            //
            // if (...) then x := 1; else x := 2;
            //                      ^
            //
            // `None` indicates the end-of-stream.
            match self.peek() {
                None
                | Some(Spanned {
                    inner: Token::RightParen | Token::Else,
                    ..
                }) => break,
                _ => {}
            };

            let second_statement = self.statement()?;

            stmt = comp_stmt!(stmt, second_statement);
        }

        // The statement block may end with a semicolon, but it's completely
        // optional.
        self.maybe_expect_token(Token::Semicolon);
        Ok(stmt)
    }

    fn statement(&mut self) -> Result<Ast, ParseError> {
        // <statement> ::= <if_stmt>
        //               | <while_stmt>
        //               | <ass_stmt>
        //               | <skip_stmt>
        //               | "(" <statement> ")"

        let (keyword, span) = match self.peek() {
            Some(Spanned { inner, span }) => (inner, span),
            None => {
                return Err(ParseError {
                    message: "Unexpected end of token stream".to_string(),
                    span: Span(self.source.len() - 1, self.source.len()),
                })
            }
        };

        match keyword {
            Token::If => self.if_stmt(),
            Token::While => self.while_stmt(),
            Token::Identifier => self.ass_stmt(),
            Token::Skip => self.skip_stmt(),
            Token::LeftParen => {
                self.advance();
                let block = self.stmt_block();
                self.expect_token(Token::RightParen)?;
                block
            }
            _ => Err(ParseError {
                message: format!("Found {keyword:?}"),
                span: span.clone(),
            }),
        }
    }

    fn if_stmt(&mut self) -> Result<Ast, ParseError> {
        // <if_stmt> ::= "if" <expression> "then" <stmt_block> "else" <stmt_block>

        self.expect_token(Token::If)?;
        let cond = self.expression()?;

        self.expect_token(Token::Then)?;
        let block_true = self.stmt_block()?;

        self.expect_token(Token::Else)?;
        let block_false = self.stmt_block()?;

        Ok(if_stmt!(cond, block_true, block_false))
    }

    fn while_stmt(&mut self) -> Result<Ast, ParseError> {
        // <while_stmt> ::= "while" <expression> "do" <stmt_block>

        self.expect_token(Token::While)?;
        let cond = self.expression()?;

        self.expect_token(Token::Do)?;
        let body = self.stmt_block()?;

        Ok(while_stmt!(cond, body))
    }

    fn ass_stmt(&mut self) -> Result<Ast, ParseError> {
        // <ass_stmt> ::= <ident> ":=" <term>

        // First thing is to get the LHS identifier.
        let span = match self.advance() {
            Some(Spanned {
                inner: Token::Identifier,
                span,
            }) => span,
            _ => {
                return Err(ParseError {
                    message: "Expected LHS Identifier".to_string(),
                    span: Span(self.source.len() - 1, self.source.len()),
                })
            }
        };
        let ident = self.source[span.0..span.1].to_string();

        self.expect_token(Token::Assign)?;

        // Now we can find the RHS.
        Ok(ass_stmt!(ident, self.expression()?))
    }

    fn skip_stmt(&mut self) -> Result<Ast, ParseError> {
        // <skip_stmt> ::= "skip"

        self.expect_token(Token::Skip)?;
        Ok(Ast::Skip)
    }

    fn expression(&mut self) -> Result<Ast, ParseError> {
        // <expression> ::= <logical_connective>

        self.logical_connective()
    }

    fn logical_connective(&mut self) -> Result<Ast, ParseError> {
        // <logical_connective> ::= <equality> ( "&" <equality> )*

        let mut expr = self.equality()?;

        while self.peek().is_some_and(|ti| ti.inner == Token::And) {
            let _operator = self.advance();
            let rhs = self.equality()?;

            expr = and!(expr, rhs);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Ast, ParseError> {
        // <equality> ::= <comparison> ( ( "=" | "!=" ) <comparison> )?

        let mut expr = self.comparison()?;

        if self
            .peek()
            .is_some_and(|ti| matches!(ti.inner, Token::Equal | Token::NotEqual))
        {
            let operator = self.advance();
            let rhs = self.comparison()?;

            expr = match operator.unwrap().inner {
                Token::Equal => eq!(expr, rhs),
                Token::NotEqual => not!(eq!(expr, rhs)),
                _ => unreachable!(),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Ast, ParseError> {
        // <comparison> ::= <term> ( ( "<=" | "<" | ">" | ">=" ) <term> )?

        let mut expr = self.term()?;

        if self.peek().is_some_and(|ti| {
            matches!(
                ti.inner,
                Token::LessEqual | Token::LessThan | Token::GreaterEqual | Token::GreaterThan
            )
        }) {
            let operator = self.advance().unwrap().inner;
            let rhs = self.term()?;

            expr = match operator {
                // a <= b == a <= b
                Token::LessEqual => less_eq!(expr, rhs),
                // a < b == !(b <= a)
                Token::LessThan => not!(less_eq!(rhs, expr)),
                // a >= b == (b <= a)
                Token::GreaterEqual => less_eq!(rhs, expr),
                // a > b == !(a <= b)
                Token::GreaterThan => not!(less_eq!(expr, rhs)),

                _ => unreachable!(),
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Ast, ParseError> {
        // <term> ::= <factor> ( ( "-" | "+" ) <factor> )*
        let mut expr = self.factor()?;

        while self
            .peek()
            .is_some_and(|spanned| matches!(spanned.inner, Token::Subtract | Token::Add))
        {
            let Spanned {
                inner: operator,
                span,
            } = self.advance().unwrap();
            let right = self.factor()?;

            expr = match operator {
                Token::Subtract => sub!(expr, right),
                Token::Add => add!(expr, right),
                _ => {
                    return Err(ParseError {
                        message: "Unexpected operator".to_string(),
                        span,
                    })
                }
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Ast, ParseError> {
        // <factor> ::= <unary> ( "*" <unary> )*
        let mut expr = self.unary()?;

        while self.peek().is_some_and(|ti| ti.inner == Token::Multiply) {
            let _operator = self.advance().unwrap().inner;
            let right = self.unary()?;

            expr = mul!(expr, right)
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Ast, ParseError> {
        // <unary> ::= ( "!" | "-" ) <unary> | <primary>

        let result = match self.peek() {
            Some(Spanned {
                inner: Token::Subtract,
                ..
            }) => {
                self.advance();
                sub!(literal!(0), self.unary()?)
            }

            Some(Spanned {
                inner: Token::Not, ..
            }) => {
                self.advance();
                not!(self.unary()?)
            }

            _ => self.primary()?,
        };

        Ok(result)
    }

    fn primary(&mut self) -> Result<Ast, ParseError> {
        // <primary> ::=  <ident>
        //              | <literal>
        //              | "(" <expression> ")"

        let Some(Spanned { inner, span }) = self.advance() else {
            // Some issues may arise if I go past self.source.len(), even though
            // it may make error annotation some time in the potential future
            // offset by 1 for this specific issue.
            return Err(ParseError {
                message: "Reached end of token stream".to_string(),
                span: Span(self.source.len() - 1, self.source.len()),
            })
        };

        let result = match inner {
            Token::LeftParen => {
                let expr = self.expression()?;
                self.expect_token(Token::RightParen)?;
                expr
            }

            Token::True => Ast::True,
            Token::False => Ast::False,

            Token::Identifier => {
                let ident_name = self.source[span.0..span.1].to_string();
                Ast::Ident(ident_name)
            }

            Token::Literal => {
                let literal_str = &self.source[span.0..span.1];
                Ast::Literal(literal_str.parse().unwrap())
            }

            _ => {
                return Err(ParseError {
                    message: format!("Got unexpected {:?} at the primary parsing stage", inner),
                    span,
                })
            }
        };

        Ok(result)
    }
}
