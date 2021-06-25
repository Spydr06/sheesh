use crate::lex;

pub fn run_input(input: &mut String) -> i32 {
    let exit_code = 0;

    if input.trim().is_empty() {
        return 0;
    }

    input.push('\0');

    println!("Executing {:?}", input);
    let tokens = lex::lex_tokens(input.to_string()).expect("Error lexing tokens: ");
    println!("Tokens: {:?}", tokens);

    exit_code
}