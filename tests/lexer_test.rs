use while_tools::lexer::*;

/// Check that the lexer works for basic inputs.
#[test]
fn test_basic_lexing() {
    let lexer = Lexer::new("x := 1;".chars());
    let tokens: Vec<_> = lexer.map(|token| token.unwrap().inner).collect();

    assert_eq!(
        tokens,
        vec![
            Token::Identifier,
            Token::Whitespace,
            Token::Assign,
            Token::Whitespace,
            Token::Literal,
            Token::Semicolon
        ]
    )
}

/// Check that the `Err(ParseError { .. })` variant of `Result` is being
/// returned when encountering an unknown character in the stream.
#[test]
fn test_err_variant() {
    let lexer = Lexer::new("x : 1;|".chars());
    let tokens: Vec<_> = lexer.collect();

    // The lexer should return a ParseError at `:`.
    assert!(tokens[2].is_err());

    // The lexer should return a ParseError at `|`.
    assert!(tokens[6].is_err());
}

/// Check that the spans returned within the ParseError are correct.
#[test]
fn test_parse_error_spans() {
    let lexer = Lexer::new("x : 1;|".chars());
    let tokens: Vec<_> = lexer.collect();

    // The lexer should return a ParseError at `:`.
    assert!(tokens
        .get(2)
        .unwrap()
        .as_ref()
        .is_err_and(|err| err.span == Span(2, 3)));

    // The lexer should return a ParseError at `|`.
    assert!(tokens
        .get(6)
        .unwrap()
        .as_ref()
        .is_err_and(|err| err.span == Span(6, 7)));
}
