//! A module for the abstract syntax tree (AST) that many tools will use.

mod macros;

/// A recursive type representing the AST. The AST itself stores no
/// functionality or logic on how each node behaves -- that is the purview
/// of other tools that use the AST, like the interpreter.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Ast {
    // Literals and identifiers
    True,
    False,
    Literal(i32),
    Ident(String),

    // Comparison and equality
    Not {
        expr: Box<Self>,
    },
    Eq {
        left: Box<Self>,
        right: Box<Self>,
    },
    LessEq {
        left: Box<Self>,
        right: Box<Self>,
    },
    And {
        left: Box<Self>,
        right: Box<Self>,
    },

    // Arithmetic
    Add {
        left: Box<Self>,
        right: Box<Self>,
    },
    Sub {
        left: Box<Self>,
        right: Box<Self>,
    },
    Mul {
        left: Box<Self>,
        right: Box<Self>,
    },

    // Statements
    Comp {
        first: Box<Self>,
        second: Box<Self>,
    },
    Ass {
        ident: String,
        value: Box<Self>,
    },
    Skip,
    If {
        cond: Box<Self>,
        true_path: Box<Self>,
        false_path: Box<Self>,
    },
    While {
        cond: Box<Self>,
        body: Box<Self>,
    },
}
