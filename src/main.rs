use std::{
    process, 
    env,
    io::{
        self, 
        Write
    }
};

mod shell;
mod token;
mod command;
mod evaluate;
mod environment;
mod builtin;

use shell::Error;
use environment::Environment;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut env = Environment::new(env::vars());

    if args.len() == 0 {
        repl(&mut env);
    }
    else {
        let mut last_exit_code = 0;
        for arg in args {
            let res = shell::run_script(arg, &mut env);
            if let Err(err) = res {
                if let Error::EarlyExit(exit_code) = err {
                    process::exit(exit_code)
                }
                eprintln!("{}", err);
            }
            else {
                last_exit_code = res.unwrap();
            }
        }
        process::exit(last_exit_code);
    }
}

fn repl(mut env: &mut Environment) {
    let mut last_exit_code = 0;

    loop {
        // print the prompt
        print!("({}) >>> ", last_exit_code);
        io::stdout().flush().expect("error while flushing stdout");

        // get the user input
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // execute the code
        match shell::execute(input, &mut env) {
            Err(err) => {
                if let Error::EarlyExit(exit_code) = err {
                    process::exit(exit_code)
                }
                eprintln!("{}", err);
            }
            Ok(exit_code) => {
                last_exit_code = exit_code;
            }
        }        
    }
}