use crate::lexer;

// the default routine to execute shell commands
pub fn run_input(input: &mut String) -> i32 {
    if input.trim().is_empty() {
        return 0;
    }

    input.push('\0');
    let tokens = lexer::lex_tokens(input.to_string()).expect("Error lexing tokens: ");
    println!("Tokens: {:?}", tokens);

    0
}