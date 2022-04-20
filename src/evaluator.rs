use crate::{ast, environment::Environment};
use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {

} 

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "RuntimeError")
        }
    }
}

pub fn evaluate(ast: ast::Node, env: &mut Environment) -> Result<i32, RuntimeError> {
    for var in env.get_vars() {
        println!("{}: `{}'", var.get_name(), var.get_value().get_name())
    }

    return Ok(0)
}