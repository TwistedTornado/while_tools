#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Token {
    // Groupings
    LeftParen,
    RightParen,

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
    LessEqual,
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