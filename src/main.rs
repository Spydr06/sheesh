use std::{
    process,
    env
};

use environment::{Variable, Environment};

mod io;
mod shell;
mod lexer;
mod ast;
mod parser;
mod evaluator;
mod environment;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

   // disable_ctrlc();

    let mut scripts = Vec::<String>::new();
    for arg in args {
        if arg.starts_with("-") || arg.starts_with("--") {
            match &*arg {
                "-h" | "--help" =>  {
                    help();
                }   
                "-i" | "--info" => {
                    info();
                }
                "-v" | "--version" => {
                    version();
                }
                _ => {
                    eprintln!("Unknown parameter {:?}, use \"--help\" to get help.", arg);
                    process::exit(1);
                }
            }
        }
        else {
            scripts.push(arg);
        }
    }

    let mut environment = Environment::new();
    for var in env::vars() {
        let mut value = ast::Node::new(ast::NodeKind::ID);
        value.set_name(var.1);
        environment.add(Variable::Export { 
            name: var.0, 
            value
        });
    }

    if scripts.len() == 0 {
        // enter interactive mode (REPL)
        io::repl(">>> ".to_string(), &mut environment);
        process::exit(0);
    }
     
    // execute the specified scripts
    for path in scripts {
        let exit_code = shell::run_input(&mut io::read_file(&*path), &mut environment);
        println!("\"{}\" terminated with exit code {}", path, exit_code);
        if exit_code != 0 {
            process::exit(exit_code);
        }
    }
}

fn help() {
    let help_text = "";
    println!("{}", help_text);

    process::exit(0);
}

fn info() {
    let info_text = "";
    println!("{}", info_text);

    process::exit(0);
}

fn version() {
    println!("sheesh version: {}", env!("CARGO_PKG_VERSION"));

    process::exit(0);
}

fn disable_ctrlc() {
    ctrlc::set_handler(move || { /* do nothing here */}).expect("Error setting Ctrl-C handler");
}