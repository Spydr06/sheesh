use crate::{
    shell::Error,
    token::{
        Token, 
        TokenKind::{
            self, *
        }
    }
};

#[derive(Debug, Clone)]
pub enum Value {
    PlainText(TokenKind, String),
    String(String),
    Variable(String),
}

impl Value {
    fn parse(tok: &Token) -> Self {
        match tok.kind() {
            STRING => Value::String(tok.value().clone()),
            VARIABLE => Value::Variable(tok.value()[1..].to_string()),
            _ => Value::PlainText(tok.kind(), tok.value().clone())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub callee: Value,
    pub args: Vec<Value>,
    pub run_in_bg: bool
}

impl Command {
    fn parse(tokens: &mut std::slice::Iter<Token>, callee: Value) -> Self {
        let mut cmd = Command {
            callee,
            args: Vec::new(),
            run_in_bg: false
        };

        while let Some(tok) = tokens.peekable().next() {
            match tok.kind() {
                NEWLINE | SEMICOLON => {
                    break;
                }
                BGPROCESS => {
                    cmd.run_in_bg = true;
                    break;
                }
                _ => cmd.args.push(Value::parse(&tok))
            }
        }

        cmd
    }

    pub fn is_in_background(&self) -> bool {
        self.run_in_bg
    }

    pub fn combine(&mut self, other: &Command) -> &Self {
        self.args.append(&mut other.args.clone());
        self
    }

    pub fn is_keyword<'a>(&self, keyword: &'a str) -> bool {
        match self.callee.clone() {
            Value::PlainText(_, text) => {
                if text.eq(keyword) {
                    if self.args.len() != 0 {
                        eprintln!("{}: too many arguments", keyword)
                    }
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Command>, Error> {
    let mut iter = tokens.iter();
    let mut commands = Vec::new();
 
    while let Some(tok) = iter.next() {
        match tok.kind() {
            EOF => {
                break;
            }
            NEWLINE => {
                continue;
            }
            _ => {
                commands.push(Command::parse(&mut iter, Value::parse(&tok)));
            }
        }
    }

    Ok(commands)
}