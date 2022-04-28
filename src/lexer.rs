use std::{
    collections::HashMap,
    fmt
};
use lazy_static::lazy_static;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenKind {
    // Primitives
    ID,     // identifier
    VAR,    // $variable
    STRING, // "strings"
    
    // Symbols
    PIPE,      // |
    GT,        // >
    COMMA,     // ,
    SEMICOLON, // ;
    EQUALS,    // =
    LPAREN,    // (   
    RPAREN,    // )

    // Keywords
    DEF,
    END,
    IF,
    AND,
    OR,
    IN,
    FOR,
    ELSE,
    ALIAS,
    HELP,

    // Escape characters
    NEWLN,
    EOF
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::ID => "identifier",
            Self::VAR => "variable",
            Self::STRING => "string",
            Self::PIPE => "|",
            Self::GT => ">",
            Self::COMMA => ",",
            Self::SEMICOLON => ";",
            Self::EQUALS => "=",
            Self::DEF => "def",
            Self::END => "end",
            Self::IF => "if",
            Self::AND => "and",
            Self::OR => "or",
            Self::IN => "in",
            Self::FOR => "for",
            Self::ELSE => "else",
            Self::ALIAS => "alias",
            Self::HELP => "help",
            Self::NEWLN => "newline",
            Self::EOF => "end of file",
            Self::LPAREN => "(",
            Self::RPAREN => ")",
        })
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenKind> = {
        let mut map = HashMap::new();
        map.insert("def".to_string(), TokenKind::DEF);
        map.insert("end".to_string(), TokenKind::END);
        map.insert("if".to_string(), TokenKind::IF);
        map.insert("else".to_string(), TokenKind::ELSE);
        map.insert("alias".to_string(), TokenKind::ALIAS);
        map.insert("help".to_string(), TokenKind::HELP);
        map.insert("and".to_string(), TokenKind::AND);
        map.insert("or".to_string(), TokenKind::OR);
        map.insert("in".to_string(), TokenKind::IN);
        map.insert("for".to_string(), TokenKind::FOR);
        map
    };
}


#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenKind,
    val: String,
}

#[macro_export]
macro_rules! next {
    ($iter:expr) => {
        $iter.next().unwrap_or('\0')
    };
}

#[macro_export]
macro_rules! tok {
    ($kind:ident, $val:expr) => {
        Token::new(TokenKind::$kind, $val.to_string())
    };

    ($kind:expr, $val:expr) => {
        Token::new($kind, $val.to_string())
    };
}

const WHTIESPACES: &str = " \t\r\n";
const OPERATORS: &str = ";,()";

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

                Ok(tok!(STRING, input[0..len]))
            }

            '$' => {
                let mut len = 1;
                c = next!(chars);
                while c.is_alphanumeric() || c == '_' {
                    len += 1;
                    c = next!(chars);
                }

                Ok(tok!(VAR, input[0..len]))
            }

            '>' => Ok(tok!(GT, ">")),
            ',' => Ok(tok!(COMMA, ",")),
            ';' => Ok(tok!(SEMICOLON, ";")),
            '|' => Ok(tok!(PIPE, "|")),
            '=' => Ok(tok!(EQUALS, "=")),
            '(' => Ok(tok!(LPAREN, "(")),
            ')' => Ok(tok!(RPAREN, ")")),
            
            '\n' => { Ok(tok!(NEWLN, "newline")) }
            '\0' => { Ok(tok!(EOF, "end of file")) }

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
                    if WHTIESPACES.contains(c) || OPERATORS.contains(c) {
                        break;
                    }

                    len += 1;
                    c = next!(chars);
                }

                Ok(tok!(*KEYWORDS.get(&input[0..len]).unwrap_or(&TokenKind::ID), input[0..len]))
            }
        }
    }
}

fn skip_whitespace(input: String) -> usize {
    let mut iter = input.chars();
    let mut i = 0;

    while WHTIESPACES.contains(iter.next().unwrap()) {
        i += 1;
    }

    i
}

pub fn lex_tokens(input: String) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::<Token>::new();
    let mut c = 0;

    loop {
        c += skip_whitespace(input[c..].to_string());

        let tok_res = Token::get_token(input[c..].to_string());
        if tok_res.is_err() {
            eprintln!("{}", tok_res.as_ref().unwrap_err());
            return Err("er")
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