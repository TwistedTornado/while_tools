use std::fs::File;
use std::io::Read;
use std::process::exit;
use while_tools::interpreter::Interpreter;

use while_tools::lexer::Lexer;
use while_tools::parser::Parser;

fn main() {
    // This is a basic example showing the pipeline as to how the source is
    // processed.

    // Getting a handle on the file.
    println!("Opening file...");

    let file_path = "examples/gcd.while";

    let mut file = match File::open(file_path) {
        Ok(f) => {
            println!("File opened successfully.");
            f
        }
        Err(e) => {
            println!("{e}");
            exit(0);
        }
    };

    // Reading the file into a string, assuming valid UTF-8

    let mut source_string = String::new();

    println!("\nReading file...");

    match file.read_to_string(&mut source_string) {
        Ok(b) => {
            println!("File read successfully ({b} bytes).")
        }
        Err(e) => {
            println!("Failed reading `{file_path}` to string: {e}");
            exit(0)
        }
    }

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
    println!("\nLexing...");

    let tokens = Lexer::new(source_string.chars());

    let result: Vec<_> = match tokens.collect::<Result<Vec<_>, _>>() {
        Ok(v) => {
            println!("Lexed successfully ({} tokens).", v.len());
            v
        }
        Err(e) => {
            println!("{e}");
            exit(0);
        }
    };

    // Now that we have an errorless stream of tokens, we parse. The parser needs
    // access to the source so that it can extract identifier names and literal
    // values and add them into the AST.
    println!("\nParsing...");
    let mut parser = Parser::new(&source_string, result);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(msg) => {
            println!("Error: {msg}");
            exit(0)
        }
    };

    println!("Parsed.");

    // The next step is to interpret this AST.

    println!("\nInterpreting...");

    let result = Interpreter::new(ast).interpret();

    match result {
        Ok(state) => println!("Interpreted. {:?}", state),
        Err(e) => println!("{e}"),
    }
}
