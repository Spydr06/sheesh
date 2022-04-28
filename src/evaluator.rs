use crate::{
    ast, 
    environment::{
        Environment, 
        Variable, 
        Callable
    }
};
use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    UnknownAction(ast::NodeKind),
    CommandNotFound(String),
    Unimplemented(&'static str),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownAction(kind) => write!(f, "Unknown action `{:?}'", kind),
            Self::CommandNotFound(cmd) => write!(f, "sheesh: command not found `{}'", cmd),
            Self::Unimplemented(desc) => write!(f, "Unimplemented: `{}'", desc)
        }
    }
}

pub fn evaluate(ast: &ast::Node, env: &mut Environment) -> Result<(i32, String), RuntimeError> {
    use {ast::NodeKind::*, self::RuntimeError::*};

    match ast.get_kind() {
        ROOT => {
            let mut last_val = (0, String::new());
            for stmt in ast.get_nodes() {
                let result = evaluate(stmt, env);
                if result.is_err() {
                    return result
                }
                last_val = result.unwrap();
            };
            Ok(last_val)
        }

        NOOP => Ok((0, String::new())),

        ALIAS => {
            let value = ast.get_nodes().get(0).unwrap();
            let name = ast.get_name();
            env.add_callable(name.to_string(), Callable::Alias {
                name: name.to_string(),
                value: value.clone()
            });

            Ok((0, String::new()))
        }

        VAR => {
            if let Some(var) = env.get_var(ast.get_name()) {
                return match var {
                    Variable::Export { value, .. } => Ok((0, value)),
                };
            }

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

            let call_name = ast.get_name().to_string();
            if let Some(callable) = env.get_callable(&call_name) {
                call(callable, args, env)
            }
            else {
                subprocess(call_name, args)
            }
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

fn call(callable: Callable, args: Vec<String>, env: &mut Environment) -> Result<(i32, String), RuntimeError> {
    match callable {
        Callable::Alias{value, ..} => evaluate(&value, env),
        Callable::Function {..} => Err(RuntimeError::Unimplemented("calling functions")),
        Callable::Builtin {func, ..} => Ok((
            func(args, env), 
            String::new()
        ))
    }
}