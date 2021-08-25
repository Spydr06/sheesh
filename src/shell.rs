use crate::lexer;
use crate::parser;
use crate::eval;

// the default routine to execute shell commands
pub fn run_input(input: &mut String) -> i32 {
    if input.trim().is_empty() {
        return 0;
    }

    input.push('\0');
    let tokens = lexer::lex_tokens(input.to_string()).expect("Error lexing tokens: ");
    println!("Tokens: {:?}", tokens);

    let ast_res = parser::parse_ast(tokens);
    if ast_res.is_err() {
        eprintln!("{}", ast_res.err().unwrap());
        return 1;
    }

    let ast = ast_res.ok().unwrap();
    println!("AST: {:?}", ast);

    let exit_res = eval::evaluate(ast);
    if exit_res.is_err() {
        eprintln!("{}", exit_res.err().unwrap());
        return 1;
    }

    exit_res.ok().unwrap()
}