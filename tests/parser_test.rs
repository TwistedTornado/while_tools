use while_tools::ast::Ast;
use while_tools::lexer::Lexer;
use while_tools::parser::*;
use while_tools::{
    ass_stmt, binary_node, comp_stmt, ident, if_stmt, less_eq, literal, literal_true, skip_stmt,
    while_stmt,
};

/// Almost like a procedural macro to make the pipeline of building an AST
/// more dry.
fn assert_ast(source: &str, ast: &Ast) {
    let tokens = Lexer::new(source.chars());
    let result: Vec<_> = match tokens.collect::<Result<Vec<_>, _>>() {
        Ok(v) => v,
        Err(e) => {
            panic!("{e}");
        }
    };

    let mut parser = Parser::new(source, result);

    let parsed_ast = match parser.parse() {
        Ok(ast) => ast,
        Err(msg) => {
            panic!("{msg}");
        }
    };

    assert_eq!(parsed_ast, *ast)
}

/// Check that the parser works for assignments.
#[test]
fn test_parsing_assignment() {
    assert_ast("x := 1;", &ass_stmt!("x".to_string(), literal!(1)));
}

/// Check that the parser works for if statements.
#[test]
fn test_parsing_if() {
    assert_ast(
        "if x <= 5 then x := 1 else x := 0",
        &if_stmt!(
            less_eq!(ident!("x".to_string()), literal!(5)),
            ass_stmt!("x".to_string(), literal!(1)),
            ass_stmt!("x".to_string(), literal!(0))
        ),
    );
}

/// Check that the parser works for while statements.
#[test]
fn test_parsing_while() {
    assert_ast(
        "while x <= 5 do skip",
        &while_stmt!(less_eq!(ident!("x".to_string()), literal!(5)), skip_stmt!()),
    );
}

/// Check that the parser works for various applications of parentheses. This
/// is the standard, sensible application of parentheses.
#[test]
fn test_parsing_various_parens_standard() {
    let ast = while_stmt!(less_eq!(ident!("x".to_string()), literal!(5)), skip_stmt!());

    assert_ast("while (x <= 5) do (skip)", &ast);
}

/// Check that the parser works for various applications of parentheses. This test overloads
/// the outside of the statement.
#[test]
fn test_parsing_various_parens_around() {
    let ast = while_stmt!(less_eq!(ident!("x".to_string()), literal!(5)), skip_stmt!());

    assert_ast("((((while (x <= 5) do ((skip))))))", &ast);
}

/// Check that the parser works for various applications of parentheses. This
/// test overloads the condition but not the body.
#[test]
fn test_parsing_various_parens_cond() {
    let ast = while_stmt!(less_eq!(ident!("x".to_string()), literal!(5)), skip_stmt!());

    assert_ast("while (((x <= 5))) do skip", &ast);
}

/// Check that the parser works for various applications of parentheses. This
/// wraps all possible things that could be wrapped.
#[test]
fn test_parsing_various_parens_everywhere() {
    let ast = while_stmt!(less_eq!(ident!("x".to_string()), literal!(5)), skip_stmt!());

    assert_ast("(while ((x) <= (5)) do (skip))", &ast);
}

/// Check that the parser parses as much as it can, even if it meets (S1); S2.
#[test]
fn test_parsing_statement_block_greediness() {
    let ast = while_stmt!(Ast::True, comp_stmt!(skip_stmt!(), skip_stmt!()));

    assert_ast("while true do (skip); skip", &ast);
}

/// Check that the parser stops parsing when the entire statement is wrapped in
/// parentheses.
#[test]
fn test_parsing_statement_block_paren_respect() {
    let ast = comp_stmt!(while_stmt!(literal_true!(), skip_stmt!()), skip_stmt!());

    assert_ast("(while true do skip); skip", &ast);
}
