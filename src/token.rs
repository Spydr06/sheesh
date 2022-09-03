use crate::shell::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // any identifier
    GENERIC,
    VARIABLE, // $identifier

    // strings '<str>' "<str>"
    STRING,

    // symbols
    SEMICOLON, // ;
    ASSIGN,    // =
    BGPROCESS, // &
    AND,       // &&
    PIPE,      // |
    OR,        // ||
    INSTREAM,  // <
    OUTSTREAM, // >

    NEWLINE, // new line
    EOF // end of file
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    value: String
}

const SYMBOLS: &str = "=&|;<>\n";

macro_rules! char_token {
    ($kind:ident,$ch:expr) => {
        Ok(Self {
            kind: TokenKind::$kind,
            value: String::from($ch)
        })
    };
}

impl Token {
    fn eof() -> Self {
        Self {
            kind: TokenKind::EOF,
            value: String::new()
        }
    }

    fn get_token(input: String) -> Result<Self, Error> {
        let mut chars = input.chars();

        let r = chars.next();
        if r.is_none() {
            return Ok(Self::eof())
        }
        let c = r.unwrap();

        match c {
            '\0' => Ok(Self::eof()),
            '\n' => char_token!(NEWLINE, c),
            ';' => char_token!(SEMICOLON, c),
            '=' => char_token!(ASSIGN, c),
            '>' => char_token!(OUTSTREAM, c),
            '<' => char_token!(INSTREAM, c),

            '&' => {
                if let Some(next) = chars.next() && next == '&' {
                    Ok(Token { 
                        kind: TokenKind::AND, 
                        value: String::from("&&")
                    })
                }
                else {
                    char_token!(BGPROCESS, c)
                }
            }

            '|' => {
                if let Some(next) = chars.next() && next == '|' {
                    Ok(Token { 
                        kind: TokenKind::OR, 
                        value: String::from("||")
                    })
                }
                else {
                    char_token!(PIPE, c)
                }
            }

            '\'' | '\"' => {
                let len = skip_until(chars, |ch| ch == c);
                if let Err(_) = len {
                    return Err(Error::SyntaxError("Unterminated `'` string"))
                }

                Ok(Self {
                    kind: TokenKind::STRING,
                    value: input[0..len.unwrap() + 2].to_string()
                })
            }

            _ => {
                let len = match skip_until(chars, |ch| is_whitespace(ch) || SYMBOLS.contains(ch)) {
                    Ok(v) => v,
                    Err(v) => v
                };

                Ok(Self {
                    kind: if c == '$' { 
                        TokenKind::VARIABLE 
                    } else { 
                        TokenKind::GENERIC 
                    },
                    value: input[0..len + 1].to_string()
                })
            }
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    fn is_eof(&self) -> bool {
        self.kind == TokenKind::EOF
    }

    fn len(&self) -> usize {
        self.value.len()
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}

fn skip_until<F>(mut chars: std::str::Chars, cmp: F) -> Result<usize, usize> 
    where F: Fn(char) -> bool
{
    let mut i = 0usize;
    while !cmp(chars.next().ok_or(i)?) {
        i += 1;
    }

    Ok(i)
}

fn is_whitespace(c: char) -> bool {
    " \t\r".contains(c)
}

fn skip_whitespace(input: String) -> usize {
    match skip_until(input.chars(), |ch| !is_whitespace(ch)) {
        Ok(v) => v,
        Err(v) => v
    }
}

pub fn tokenize(input: String) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut c = 0usize;

    loop {
        c += skip_whitespace(input[c..].to_string());
        let tok = Token::get_token(input[c..].to_string())?;
        c += tok.len();

        tokens.push(tok.clone());

        if tok.is_eof() {
            break;
        }
    }

    Ok(tokens)
}