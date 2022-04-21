use std::{
    process,
    fs::File,
    io::{
        self,
        Write,
        Read,
    },
};

use crate::{
    shell,
    environment::Environment
};

pub fn repl(prompt: String, environment: &mut Environment) {
    loop {
        // print the prompt
        print!("{}", prompt);
        std::io::stdout().flush().expect("error while printing to stdout");

        // get the user input
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // run the implemented prompt
        shell::run_input(&mut input, environment);
    }
}

pub fn read_file(path: &str) -> String {
    match File::open(path) {
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();

            return buf;
        },
        
        Err(error) => {
            eprintln!("Error opening file {}: {}", path, error);
            process::exit(1);
        },
    }
}