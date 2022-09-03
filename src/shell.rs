use crate::{
    token,
    command,
    evaluate::evaluate,
    environment::Environment
};

use std::{
    fmt, 
    fs::File, 
    io::Read,
    env,
    path::Path
};

#[derive(Debug)]
pub enum Error {
    ReadFile(String, String),
    SyntaxError(&'static str),
    CommandNotFound(String),
    WrongNumOfArgs(&'static str, usize, usize),
    EarlyExit(i32)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReadFile(path, err) => write!(f, "Error reading file \"{}\": {}", path, err),
            Self::SyntaxError(err) => write!(f, "Syntax error: {}", err),
            Self::CommandNotFound(exec) => write!(f, "Command not found: {}", exec),
            Self::WrongNumOfArgs(exec, expected, received) => write!(f, "{}: expect {} arguments, got {}", exec, expected, received),
            Self::EarlyExit(code) => write!(f, "Process exited with code {}.", code)
        }
    }
}

pub fn execute(input: String, env: &mut Environment) -> Result<i32, Error> {
    let tokens = token::tokenize(input)?;

    //println!("{:?}", tokens);

    let commands = command::parse(tokens)?;
    //println!("{:?}", commands);

    evaluate(commands, env)
}

fn read_file(path: String) -> Result<String, Error> {
    match File::open(path.clone()) {
        Ok(mut file) => {
            let mut buf = String::new();
            if let Err(err) = file.read_to_string(&mut buf) {
                Err(Error::ReadFile(path, err.to_string()))
            }
            else {
                Ok(buf)
            }
        }

        Err(err) => {
            Err(Error::ReadFile(path, err.to_string()))
        }
    }
}

pub fn run_script(path: String, env: &mut Environment) -> Result<i32, Error> {
    let input = read_file(path)?;
    execute(input, env)
}

pub fn set_directory(path: &String) -> Result<i32, Error> {
    match env::set_current_dir(Path::new(path)) {
        Err(err) => {
            eprintln!("cd: {}", err);
            Ok(1)
        }
        Ok(_) => Ok(0)
    }
}