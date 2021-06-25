use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenKind {
    ID,
    INT,
    CHAR,
    STRING,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    DEF,
    END,
    IF,
    ELSE,
    FOR,
    IN,
    ALIAS,

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
        map.insert("for".to_string(), TokenKind::FOR);
        map.insert("in".to_string(), TokenKind::IN);
        map.insert("alias".to_string(), TokenKind::ALIAS);
        map
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
        let mut iter = input.chars();
        let char = iter.next().unwrap();

        if char.is_digit(10) {
            let mut num_end = 1;
            
            while iter.next().unwrap().is_digit(10) {
                num_end += 1;
            }
            
            return Ok(Token::new(TokenKind::INT, input[0..num_end].to_string()));
        }

        if char.is_alphabetic() {
            let mut id_end = 1;
            while iter.next().unwrap().is_alphanumeric() {
                id_end += 1;
            }
            let val = input[0..id_end].to_string();

            if KEYWORDS.contains_key(&val) {
                return Ok(Token::new(*KEYWORDS.get(&val).unwrap(), val))
            }

            return Ok(Token::new(TokenKind::ID, val));
        }

        match char {
            '(' => {
                return Ok(Token::new(TokenKind::LPAREN, '('.to_string()));
            }
            ')' => {
                return Ok(Token::new(TokenKind::RPAREN, ')'.to_string()));
            }
            '{' => {
                return Ok(Token::new(TokenKind::LBRACE, '{'.to_string()));
            }
            '}' => {
                return Ok(Token::new(TokenKind::RBRACE, '}'.to_string()));
            }
            '[' => {
                return Ok(Token::new(TokenKind::LBRACKET, '['.to_string()));
            }
            ']' => {
                return Ok(Token::new(TokenKind::RBRACKET, ']'.to_string()));
            }

            '"' => {

            }

            ''' => {
                
            }

            '\0' => {
                return Ok(Token::new(TokenKind::EOF, "EOF".to_string()));
            }
            _ => {
                return Err(format!("Unknown token '{}'", char));
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
            return Err(tok_res.err().unwrap());
        }

        let tok = tok_res.unwrap();
        c += tok.get_val().len();
        tokens.push(tok.clone());
        println!("Token: {:?}", tok);

        if tok.get_kind() == TokenKind::EOF {
            break;
        }
    }

    Ok(tokens)
}