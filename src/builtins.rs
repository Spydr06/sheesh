use crate::environment::*;

use std::{
    collections::HashMap,
    process
};
use lazy_static::lazy_static;

pub type BuiltinFunc = fn(Vec<String>, env: &mut Environment) -> i32;

fn exit(args: Vec<String>, _env: &mut Environment) -> i32 {
    match args.len() {
        0 => process::exit(0),
        1 => {
            let code = args
                .first()
                .unwrap_or(&"0".to_string())
                .parse::<i32>();
            
            if let Ok(c) = code {
                process::exit(c)
            }
            else {
                eprintln!("sheesh: exit: numeric argument required");
                1
            }
        },
        _ => {
            eprintln!("sheesh: exit: too many arguments");
            1
        }
    }
}

fn exec(_args: Vec<String>, _env: &mut Environment) -> i32 {
    eprintln!("sheesh: exec: not implemented yet");
    1
}

fn cd(_args: Vec<String>, _env: &mut Environment) -> i32 {
    eprintln!("sheesh: cd: not implemented yet");
    1
}

fn pushd(_args: Vec<String>, _env: &mut Environment) -> i32 {
    eprintln!("sheesh: pushd: not implemented yet");
    1
}

fn popd(_args: Vec<String>, _env: &mut Environment) -> i32 {
    eprintln!("sheesh: popd: not implemented yet");
    1
}

lazy_static! {
    static ref BUILTINS: HashMap<String, BuiltinFunc> = {
        let mut map = HashMap::new();
        map.insert("exit" .to_string(), exit  as BuiltinFunc);
        map.insert("exec" .to_string(), exec  as BuiltinFunc);
        map.insert("cd"   .to_string(), cd    as BuiltinFunc);
        map.insert("pushd".to_string(), pushd as BuiltinFunc);
        map.insert("popd" .to_string(), popd  as BuiltinFunc);
        map
    };
}

pub fn register(env: &mut Environment) {
    for (name, func) in BUILTINS.iter() {
        let builtin = Callable::Builtin { 
            name: name.to_string(), 
            func: *func 
        };
        env.add_callable(name.to_string(), builtin);
    }
}