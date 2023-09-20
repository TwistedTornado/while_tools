/// A token resulting from the lexing process. They are stateless but may
/// identify any portion of the source using [`Span`][span] or
/// [`Spanned<T>`][spanned_t].
///
/// [span]: crate::lexer::Span
/// [spanned_t]: crate::lexer::Spanned
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Token {
    // Groupings
    LeftParen,
    RightParen,
    LeftSemantic,
    RightSemantic,

    // Arithmetic operators
    Add,
    Subtract,
    Multiply,

    // Literals
    Literal,
    Identifier,
    True,
    False,

    // Comparison and equality operators
    Equal,
    NotEqual,
    LessEqual,
    LessThan,
    GreaterEqual,
    GreaterThan,
    Not,
    And,

    // Statement identifiers
    Assign,
    If,
    Then,
    Else,
    While,
    Do,
    Skip,

    // Miscellaneous symbols
    Whitespace,
    LineBreak,
    Unknown,
    Semicolon,
}
