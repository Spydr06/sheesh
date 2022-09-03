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

#[derive(Debug, Clone)]
pub struct Command {
    pub callee: String,
    pub args: Vec<Value>,
    pub run_in_bg: bool
}

impl Command {
    fn parse(tokens: &mut std::slice::Iter<Token>, name: String) -> Self {
        let mut cmd = Command {
            callee: name,
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
                STRING => {
                    cmd.args.push(Value::String(tok.value().clone()))
                }
                VARIABLE => {
                    cmd.args.push(
                        Value::Variable(
                            tok.value()[1..]
                            .to_string()
                        )
                    )
                }
                _ => {
                    cmd.args.push(Value::PlainText(tok.kind(), tok.value().clone()))
                }
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
                commands.push(Command::parse(&mut iter, tok.value().clone()));
            }
        }
    }

    Ok(commands)
}