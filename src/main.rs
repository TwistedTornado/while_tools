use while_tools::lexer::Lexer;

fn main() {
    // A basic example showing what information the lexer may return for a
    // token. Although each element contains a substantial amount of nesting,
    // it encodes a lot of information and takes advantage of idiomatic
    // Rust structures.
    let tokens: Vec<_> = Lexer::new("skip".chars()).collect();
    println!("{:#?}", tokens);
}
