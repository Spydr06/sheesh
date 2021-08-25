use crate::lexer;
use crate::parser;

pub fn run_input(input: &mut String) -> i32 {
    let exit_code = 0;

    if input.trim().is_empty() {
        return 0;
    }

    input.push('\0');
    let tokens = lexer::lex_tokens(input.to_string()).expect("Error lexing tokens: ");
    println!("Tokens: {:?}", tokens);

    let ast = parser::parse_ast(tokens);
    println!("AST: {:?}", ast);

    exit_code
}