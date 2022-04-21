use std::collections::hash_map::{
    HashMap,
    RandomState
};

use crate::ast;

#[derive(Clone)]
pub enum Variable {
    Export {name: String, value: String},
    Alias {name: String, value: ast::Node},
}

impl Variable {
    pub fn get_name(&mut self) -> String {
        match self {
            Self::Export{name, ..} => name.to_string(),
            Self::Alias{name, ..} => name.to_string()
        }
    }

    pub fn get_value(self) -> Option<ast::Node> {
        match self {
            Self::Export{value: _, ..} => None,
            Self::Alias{value, ..} => Some(value)
        }
    }
}

pub struct Environment {
    vars: HashMap<String, Variable>,
    outer: Box<Option<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            vars: HashMap::<String, Variable, RandomState>::new(),
            outer: Box::<Option<Environment>>::new(None)
        }
    }

    pub fn add(&mut self, name: String, var: Variable) {
        self.vars.insert(name, var);
    }

    pub fn get(&self, name: &String) -> Option<Variable> {
        let found = self.vars.get(name);
        if found.is_none() {
            None
        }
        else {
            Some(found.unwrap().clone())
        }
       // if result.is_none() && self.outer.is_some() {
       //     self.outer.unwrap().get(name)
       // } else {    
       //     result
       // }
    }
}