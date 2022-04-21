use crate::{
    lexer,
    parser,
    environment::Environment,
    evaluator::evaluate
};

// the default routine to execute shell commands
pub fn run_input(input: &mut String, environment: &mut Environment) -> i32 {
    if input.trim().is_empty() {
        return 0;
    }

    input.push('\0');
    let tokens = lexer::lex_tokens(input.to_string()).expect("Error lexing tokens: ");
    //println!("Tokens: {:#?}", tokens);

    let result = parser::parse(tokens);
    if result.is_err() {
        println!("Error: {}", result.unwrap_err());
        return 1;
    }
    let ast = result.unwrap();
    //println!("AST: {:#?}", ast);

    let result = evaluate(&ast, environment);
    if result.is_err() {
        println!("Error: {}", result.unwrap_err());
        return 1;
    }

    let (exit_code, output) = result.unwrap();

    exit_code
}