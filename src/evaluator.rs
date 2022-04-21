use crate::{ast, environment::{Environment, Variable}};
use std::{fmt, io::Write, os::unix::prelude::FromRawFd};

#[derive(Debug)]
pub enum RuntimeError {
    UnknownAction(ast::NodeKind),
    CommandNotFound(String),
    UnsupportedPlatform(&'static str),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownAction(kind) => write!(f, "Unknown action `{:?}'", kind),
            Self::CommandNotFound(cmd) => write!(f, "sheesh: command not found `{}'", cmd),
            Self::UnsupportedPlatform(platform) => write!(f, "unsupported platform `{}'", platform)
        }
    }
}

pub fn evaluate(ast: &ast::Node, env: &mut Environment) -> Result<(i32, String), RuntimeError> {
    use {ast::NodeKind::*, self::RuntimeError::*};

    match ast.get_kind() {
        ROOT => {
            for stmt in ast.get_nodes() {
                let result = evaluate(stmt, env);
                if result.is_err() {
                    return result
                }
            };
            Ok((0, String::new()))
        }

        NOOP => Ok((0, String::new())),

        ALIAS => {
            let value = ast.get_nodes().get(0).unwrap();
            env.add(Variable::Alias {
                name: ast.get_name().to_string(),
                value: value.clone()
            });

            Ok((0, String::new()))
        }

        CALL => {
            let mut args = Vec::<String>::new();
            for arg in ast.get_args() {
                let result = evaluate(arg, env);
                if result.is_err() {
                    return result
                }

                args.push(result.unwrap().1);
            }

            let result = subprocess(ast.get_name().to_string(), args);
            if result.is_err() {
                return Err(result.unwrap_err());
            }

            Ok(result.unwrap())
        }

        _ => {
            Err(UnknownAction(ast.get_kind()))
        }
    }
}

fn subprocess(name: String, args: Vec<String>) -> Result<(i32, String), RuntimeError> {
    use std::process::{
        Command,
        Stdio
    };

    let mut cmd = Command::new(name.to_string());
    for arg in args {
        cmd.arg(arg);
    }

    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());
    cmd.stdin(Stdio::inherit());
    
    let result = cmd.output();
    
    if result.is_err() {
        return Err(RuntimeError::CommandNotFound(name));
    }
    
    let output = result.unwrap();
    Ok((output.status.code().unwrap_or(0), format!("{}{}", 
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )))
}
