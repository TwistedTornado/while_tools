use while_tools::ast::Ast;
use while_tools::{ass_stmt, binary_node, ident, if_stmt, less_eq, literal, skip_stmt, while_stmt};

/// Test that the generalised binary node macro works.
#[test]
fn test_ast_binary_node_macro() {
    let add_node = binary_node!(Add, literal!(1), literal!(2));

    assert_eq!(
        add_node,
        Ast::Add {
            left: Box::new(Ast::Literal(1)),
            right: Box::new(Ast::Literal(2))
        }
    );

    let less_eq_node = binary_node!(LessEq, literal!(3), literal!(4));

    assert_eq!(
        less_eq_node,
        Ast::LessEq {
            left: Box::new(Ast::Literal(3)),
            right: Box::new(Ast::Literal(4))
        }
    )
}

/// Test that ident! macro correctly produces an Ident node.
#[test]
fn test_ast_ident_macro() {
    let ident_x = ident!("x".to_string());

    assert_eq!(ident_x, Ast::Ident("x".to_string()));
}

/// Test that literal! macro correctly produces a Literal node.
#[test]
fn test_ast_literal_macro() {
    let literal_1 = literal!(1);
    assert_eq!(literal_1, Ast::Literal(1));

    let literal_negative_5 = literal!(-5);
    assert_eq!(literal_negative_5, Ast::Literal(-5));
}

/// Test that ass_stmt! macro correctly produces an Ass node.
#[test]
fn test_ast_ass_stmt_macro() {
    let ass_x_5 = ass_stmt!("x".to_string(), literal!(5));
    assert_eq!(
        ass_x_5,
        Ast::Ass {
            ident: "x".to_string(),
            value: Box::new(Ast::Literal(5))
        }
    );
}

/// Test that the if macro works.
#[test]
fn test_ast_if_macros() {
    let if_expansion = if_stmt!(
        less_eq!(literal!(1), literal!(2)),
        skip_stmt!(),
        skip_stmt!()
    );
    assert_eq!(
        if_expansion,
        Ast::If {
            cond: Box::new(Ast::LessEq {
                left: Box::new(Ast::Literal(1)),
                right: Box::new(Ast::Literal(2))
            }),
            true_path: Box::new(Ast::Skip),
            false_path: Box::new(Ast::Skip)
        }
    );
}

/// Test that the while macro works.
#[test]
fn test_ast_while_macros() {
    let while_expansion = while_stmt!(less_eq!(literal!(1), literal!(2)), skip_stmt!());
    assert_eq!(
        while_expansion,
        Ast::While {
            cond: Box::new(Ast::LessEq {
                left: Box::new(Ast::Literal(1)),
                right: Box::new(Ast::Literal(2))
            }),
            body: Box::new(Ast::Skip),
        }
    );
}

/// Test that the while macro works.
#[test]
fn test_ast_skip_macro() {
    assert_eq!(skip_stmt!(), Ast::Skip)
}
