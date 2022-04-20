use crate::ast;

pub enum Variable {
    Export {name: String, value: ast::Node},
    Alias {name: String, value: ast::Node},
}

impl Variable {
    pub fn get_name(&mut self) -> String {
        match self {
            Self::Export{name, ..} => name.to_string(),
            Self::Alias{name, ..} => name.to_string()
        }
    }

    pub fn get_value(&mut self) -> &ast::Node {
        match self {
            Self::Export{value, ..} => value,
            Self::Alias{value, ..} => value
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