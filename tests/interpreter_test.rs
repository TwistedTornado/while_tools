use while_tools::interpreter::state::State;
use while_tools::interpreter::Interpreter;
use while_tools::lexer::Lexer;
use while_tools::parser::*;

/// Almost like a procedural macro to make the pipeline of running a program
/// more DRY.
fn get_program_result(source: &str) -> State {
    let tokens = Lexer::new(source.chars());
    let result: Vec<_> = match tokens.collect::<Result<Vec<_>, _>>() {
        Ok(v) => v,
        Err(e) => {
            panic!("{e}");
        }
    };

    let mut parser = Parser::new(source, result);

    let ast_result = match parser.parse() {
        Ok(ast) => ast,
        Err(msg) => {
            panic!("{msg}");
        }
    };

    let result = Interpreter::new(ast_result).interpret().unwrap();

    result
}

/// Check that the interpreter works for if statements.
#[test]
fn test_interpret_if() {
    let result = get_program_result("if x <= 5 then x := 1 else x := 0");
    let mut expected_result = State::new();
    expected_result.set("x".to_string(), 1);
    assert_eq!(result, expected_result)
}
