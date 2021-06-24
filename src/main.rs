use std::{
    process,
    env
};

mod repl;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

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

    if scripts.len() == 0 {
        // enter interactive mode (REPL)
        repl::repl(">>> ".to_string());
    }
    else {
        // execute the specified scripts

        println!("Executing {:?}", scripts);
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