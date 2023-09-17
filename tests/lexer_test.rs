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

/// Test the automatic semicolon insertion (ASI) when there is no existing
/// semicolon.
#[test]
fn test_asi_when_no_semicolon() {
    let lexer = Lexer::new("skip\r\nskip".chars());
    let tokens: Vec<_> = lexer.map(|result| result.unwrap().inner).collect();

    assert_eq!(tokens, vec![Token::Skip, Token::Semicolon, Token::Skip]);
}

/// Test the automatic semicolon insertion (ASI) when there already is a
/// semicolon between two lines.
#[test]
fn test_asi_when_already_semicolon() {
    let lexer = Lexer::new("skip;\r\nskip".chars());
    let tokens: Vec<_> = lexer.map(|result| result.unwrap().inner).collect();

    assert_eq!(tokens, vec![Token::Skip, Token::Semicolon, Token::Skip]);
}

/// Test the automatic semicolon insertion (ASI) when the semicolon is on its
/// own line between two other lines. This should not be condensed -- we should
/// get two consecutive semicolons.
#[test]
fn test_asi_semicolon_on_own_line() {
    let lexer = Lexer::new("skip\r\n;\r\nskip".chars());
    let tokens: Vec<_> = lexer.map(|result| result.unwrap().inner).collect();

    assert_eq!(
        tokens,
        vec![Token::Skip, Token::Semicolon, Token::Semicolon, Token::Skip]
    );
}

/// Test the automatic semicolon insertion (ASI) when there are multiple
/// line-breaks between two other lines. This should be condensed into one
/// Semicolon token.
#[test]
fn test_asi_multiple_linebreaks() {
    let lexer = Lexer::new("skip\r\n\r\n\r\nskip".chars());
    let tokens: Vec<_> = lexer.map(|result| result.unwrap().inner).collect();

    assert_eq!(tokens, vec![Token::Skip, Token::Semicolon, Token::Skip]);
}

/// Test the automatic semicolon insertion (ASI) when there are consecutive
/// explicit semicolon characters. This should not be condensed -- we should
/// have that same amount of Semicolon tokens.
#[test]
fn test_asi_multiple_semicolons() {
    let lexer = Lexer::new("skip;;skip".chars());
    let tokens: Vec<_> = lexer.map(|result| result.unwrap().inner).collect();

    assert_eq!(
        tokens,
        vec![Token::Skip, Token::Semicolon, Token::Semicolon, Token::Skip]
    );
}

/// Test the automatic semicolon insertion (ASI) when there are non-Windows
/// line-endings such as simply LF (\n).
#[test]
fn test_asi_non_crlf_endings() {
    let lexer = Lexer::new("skip\nskip".chars());
    let tokens: Vec<_> = lexer.map(|result| result.unwrap().inner).collect();

    assert_eq!(tokens, vec![Token::Skip, Token::Semicolon, Token::Skip]);
}
