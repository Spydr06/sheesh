use lazy_static::__Deref;

use crate::ast;

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
            Self::Export{value, ..} => None,
            Self::Alias{value, ..} => Some(value)
        }
    }
}

pub struct Environment {
    vars: Vec<Variable>,
    outer: Box<Option<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            vars: Vec::<Variable>::new(), 
            outer: Box::<Option<Environment>>::new(None)
        }
    }

    pub fn add(&mut self, var: Variable) {
        self.vars.push(var);
    }

    pub fn get_vars(&mut self) -> &mut Vec<Variable> {
        &mut self.vars
    }
}