use std::io;
use std::io::Write;

pub fn repl(prompt: String) {
    loop {
        // print the prompt
        print!("{}", prompt);
        std::io::stdout().flush().expect("error while printing to stdout");

        // get the user input
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!(": {}", input);
    }
}