use while_tools::ast::*;
use while_tools::{binary_node, literal};

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
