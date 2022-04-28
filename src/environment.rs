use std::collections::hash_map::HashMap;

use crate::{
    ast,
    builtins::BuiltinFunc
};

#[derive(Clone)]
pub enum Variable {
    Export {name: String, value: String},
}

#[derive(Clone)]
pub enum Callable {
    Alias {name: String, value: ast::Node},
    Function {name: String, value: ast::Node, args: Vec<ast::Node>},
    Builtin {name: String, func: BuiltinFunc}
}

pub struct Environment {
    vars: HashMap<String, Variable>,
    callables: HashMap<String, Callable>,
    outer: Box<Option<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            vars: HashMap::<String, Variable>::new(),
            callables: HashMap::<String, Callable>::new(),
            outer: Box::<Option<Environment>>::new(None)
        }
    }

    pub fn add_var(&mut self, name: String, var: Variable) {
        self.vars.insert(name, var);
    }

    pub fn add_callable(&mut self, name: String, callable: Callable) {
        self.callables.insert(name, callable);
    }

    pub fn get_var(&self, name: &String) -> Option<Variable> {
        let found = self.vars.get(name);
        if found.is_none() {
            None
        }
        else {
            Some(found.unwrap().clone())
        }
    }

    pub fn get_callable(&self, name: &String) -> Option<Callable> {
        let found = self.callables.get(name);
        if found.is_none() {
            None
        }
        else {
            Some(found.unwrap().clone())
        }
    }
}