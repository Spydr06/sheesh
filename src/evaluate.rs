use crate::{
    command::{Command, Value},
    shell::Error,
    environment::{Environment, Identifier},
    builtin::*
};

use std::slice::Iter;
use subprocess::{
    PopenError,
    Exec,
    ExitStatus::*
};

fn popen_run_process(callee: &String, args: &[String], in_background: bool) -> Result<i32, PopenError> {
    let exec = if in_background { 
        Exec::cmd(callee).args(args).detached() 
    } else { 
        Exec::cmd(callee).args(args) 
    };

    let exit_status = exec.join()?;

    match exit_status {
        Exited(code) => Ok(code as i32),
        Signaled(sig) => {
            eprintln!("sheesh: {} in `{}`", sig.to_string(), callee);
            Ok(sig as i32)
        }
        Other(code) => Ok(code),
        Undetermined => Ok(0)
    }
}

impl Command {
    pub fn eval(&self, commands: &mut Iter<Command>, env: &mut Environment) -> Result<i32, Error> {
        if let Some(eval_builtin) = BUILTINS.get(self.callee.as_str()) {
            eval_builtin(self, commands, env)
        }
        else if let Some(_) = env.find_ident(&self.callee) {
            self.eval_ident_call(commands, env)
        }
        else {
            self.run_program(env) // run an external program, like ls, grep, awk, etc.
        }
    }

    fn run_program(&self, env: &mut Environment) -> Result<i32, Error> {
        let mut args = Vec::<String>::new();

        for arg in &self.args {
            args.push(arg.eval(env)?)
        }

        match popen_run_process(&self.callee, &args, self.is_in_background()) {
            Ok(exit_code) => Ok(exit_code),
            Err(pope_err) => Err(Error::CommandNotFound(pope_err.to_string()))
        }
    }

    pub fn from_args(args: &[Value], env: &mut Environment) -> Result<Self, Error> {
        Ok(Self {
            callee: args[0].eval(env)?,
            args: args[1..].to_vec(),
            run_in_bg: false
        })
    }

    fn eval_ident_call(&self, commands: &mut Iter<Command>, env: &mut Environment) -> Result<i32, Error> {
        let id = env.find_ident(&self.callee).unwrap();

        match id.clone() {
            Identifier::Alias { mut substitute } => {
                substitute.combine(self).eval(commands, env)
            },
        }
    }
}

impl Value {
    pub fn eval(&self, env: &mut Environment) -> Result<String, Error> {
        match self {
            Self::String(str) => {
                let mut chars = str.chars();
                chars.next();
                chars.next_back();
                Ok(chars.collect::<String>())
            }
            Self::PlainText(_, str) => Ok(str.to_string()),
            Self::Variable(callee) => {
                match env.find_var(callee) {
                    Some(var) => Ok(var.value()),
                    None => Ok(String::new())
                }
            }
        }
    }
}

pub fn evaluate(commands: Vec<Command>, env: &mut Environment) -> Result<i32, Error> {
    let mut last_exit_code = 0;
    let mut iter = commands.iter();
    
    while let Some(command) = iter.next() {
        last_exit_code = command.eval(&mut iter, env)?;
    }

    Ok(last_exit_code)
}