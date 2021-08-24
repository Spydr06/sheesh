use std::{collections::HashMap, process::exit};
use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenKind {
    ID,
    STRING,

    LPAREN,
    RPAREN,
    
    COMMA,

    DEF,
    END,
    IF,
    ELSE,
    ALIAS,
    EXIT,

    EOF
}

#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenKind,
    val: String,
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenKind> = {
        let mut map = HashMap::new();
        map.insert("def".to_string(), TokenKind::DEF);
        map.insert("end".to_string(), TokenKind::END);
        map.insert("if".to_string(), TokenKind::IF);
        map.insert("else".to_string(), TokenKind::ELSE);
        map.insert("alias".to_string(), TokenKind::ALIAS);
        map.insert("exit".to_string(), TokenKind::EXIT);
        map
    };
}

#[macro_export]
macro_rules! next {
    ( $iter:expr ) => {
        {
           $iter.next().unwrap_or('\0')
        }
    };
}

impl Token {
    pub fn new(kind: TokenKind, val: String) -> Self {
        Self {
            kind,
            val,
        }   
    }

    pub fn get_kind(&self) -> TokenKind {
        self.kind
    }

    pub fn get_val(&self) -> String {
        self.val.to_string()
    }

    pub fn get_token(input: String) -> Result<Token, String> {
        let mut chars = input.chars();
        let mut c = next!(chars);
        match c {
            '"' => {
                let mut len = 2;
                c = next!(chars);
                while c != '"' {
                    if c == '\0' {
                        return Err("Unterminated String Literal".to_string());
                    }
                    len += 1;
                    c = next!(chars);
                }

                Ok(Token::new(TokenKind::STRING, input[0..len].to_string()))
            }

            '(' => { Ok(Token::new(TokenKind::LPAREN, '('.to_string())) }
            ')' => { Ok(Token::new(TokenKind::RPAREN, ')'.to_string())) }
            ',' => { Ok(Token::new(TokenKind::COMMA, ','.to_string())) }
            '\0' => { Ok(Token::new(TokenKind::EOF, "EOF".to_string())) }

            _ => {
                let mut len = 0;
                loop {
                    if c == '\\' {
                        c = next!(chars);
                        len += 1;
                        if c == ' ' {
                            c = next!(chars);
                            len += 1;
                        }
                    }
                    if c.is_whitespace() {
                        break;
                    }

                    len += 1;
                    c = next!(chars);
                }

                let val = input[0..len].to_string();
                Ok(Token::new(*KEYWORDS.get(&val).unwrap_or( &TokenKind::ID), val))
            }
        }
    }
}

fn skip_whitespace(input: String) -> usize {
    let mut iter = input.chars();

    let mut i = 0;
    while iter.next().unwrap().is_whitespace() {
        i += 1;
    }

    i
}

pub fn lex_tokens(input: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::<Token>::new();
    let mut c = 0;

    loop {
        c += skip_whitespace(input[c..].to_string());

        let tok_res = Token::get_token(input[c..].to_string());
        if tok_res.is_err() {
            eprintln!("{}", tok_res.as_ref().unwrap_err());
            exit(1)
        }

        let tok = tok_res.unwrap();

        c += tok.get_val().len();
        tokens.push(tok.clone());

        if tok.get_kind() == TokenKind::EOF {
            break;
        }
    }

    Ok(tokens)
}
