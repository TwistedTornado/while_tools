use while_tools::ast::Ast;
use while_tools::lexer::Lexer;
use while_tools::{add, binary_node, literal};

fn main() {
    // A basic example showing what information the lexer may return for a
    // token. Although each element contains a substantial amount of nesting,
    // it encodes a lot of information and takes advantage of idiomatic
    // Rust structures.
    let tokens: Vec<_> = Lexer::new("skip".chars()).collect();
    println!("{:#?}", tokens);

    // This shows the expansion of the AST macros provided.
    let x = add!(literal!(5), literal!(7));
    dbg!(x);
}
