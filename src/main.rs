use std::process::exit;

use while_tools::lexer::Lexer;
use while_tools::parser::Parser;

fn main() {
    // This is a basic example showing the pipeline as to how the source is
    // processed.
    let source = "x := 5; while x <= 100 x := x + 1";

    // First is the lexing. `Lexer` itself is an `Iterator`, so we can collect
    // on it directly. But we need to check for, and handle, `LexError`s.
    // The below method only handles the first error that ever appears in the
    // stream.
    //
    // (Also note that this way, of collecting into a Vec, means it's less than
    // ideal for larger files -- we have to lex the entire source and allocate
    // for it, and then pass that on. In making the whole pipeline work on
    // iterators, we'd have to change this later on).
    //
    // In short, we need fallible iterators.
    let tokens = Lexer::new(source.chars());
    let result: Vec<_> = match tokens.collect::<Result<Vec<_>, _>>() {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            exit(0);
        }
    };

    // Now that we have an errorless stream of tokens, we parse. The parser needs
    // access to the source so that it can extract identifier names and literal
    // values and add them into the AST.
    let mut parser = Parser::new(source, result);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(msg) => {
            println!("Error: {msg}");
            exit(0)
        }
    };

    // If we didn't error, we can print out the AST. As of now, it's span-less.
    dbg!(ast);

    // This is as far as is currently implemented. The next step is to interpret
    // this AST.
}
