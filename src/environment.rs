use std::{
    collections::HashMap,
    env::Vars
};

use crate::command::Command;

pub struct Variable {
    value: String
}

impl Variable {
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Clone)]
pub enum Identifier {
    Alias {
        substitute: Command
    }
}

pub struct Environment {
    variables: HashMap<String, Variable>,     // $variables
    identifiers: HashMap<String, Identifier>, // "normal" names
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new_empty(outer: Option<Box<Environment>>) -> Self {
        Self {
            variables: HashMap::new(),
            identifiers: HashMap::new(),
            outer: outer
        }
    }

    pub fn new(vars: Vars) -> Self {
        let mut env = Self::new_empty(None);
        
        for (key, value) in vars.into_iter() {
            env.add_var(key, value);
        }
        
        env
    }

    pub fn add_var(&mut self, name: String, value: String) {
        self.variables.insert(name, Variable { value });
    }

    pub fn find_var(&self, name: &String) -> Option<&Variable> {
        if let Some(var) = self.variables.get(name) {
            Some(var)
        }
        else if let Some(outer) = &self.outer {
            outer.find_var(name)
        }
        else {
            None
        }
    }

    pub fn add_ident(&mut self, name: String, obj: Identifier) {
        self.identifiers.insert(name, obj);
    }

    pub fn find_ident(&self, name: &String) -> Option<&Identifier> {
        if let Some(var) = self.identifiers.get(name) {
            Some(var)
        }
        else if let Some(outer) = &self.outer {
            outer.find_ident(name)
        }
        else {
            None
        }
    }
}