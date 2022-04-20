use crate::{
    lexer::{
        Token,
        TokenKind::{
            self,
            *
        }
    },
    ast::{
        Node,
        NodeKind
    }
};

use std::fmt;

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedToken(Token),
    UnexpectedTokenWant(Token, TokenKind),
    UnexpectedEndOfFile(TokenKind)
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedToken(tok) => write!(f, "Unexpected token `{}'", tok.get_val()),
            Self::UnexpectedTokenWant(tok, want) => write!(f, "Unexpected token `{}', want `{}'", tok.get_val(), want),
            Self::UnexpectedEndOfFile(want) => write!(f, "Unexpected end of file, want `{}'", want)
        }
    }
}

fn expect(tok: Option<&Token>, expected: TokenKind) -> Result<&Token, SyntaxError> {
    if tok.is_none() {
        return Err(SyntaxError::UnexpectedEndOfFile(expected))
    }

    let unwrapped = tok.unwrap();

    if unwrapped.get_kind() != expected {
        return Err(SyntaxError::UnexpectedTokenWant(unwrapped.clone(), expected))
    }
    Ok(unwrapped)
}

macro_rules! next_tok {
    ($i:expr, $tokens:expr, $expect:expr) => {{
        *$i += 1;
        let got = expect($tokens.get(*$i), $expect);
        if got.is_err() {
            return Err(got.unwrap_err());
        }
        got.unwrap()
    }};
}

macro_rules! next_is {
    ($i:expr, $tokens:expr, $expected:expr) => {
        {
            let next = $tokens.get(*$i + 1);
            next.is_some() && next.unwrap().get_kind() == $expected
        }
    };
}

pub fn parse(tokens: Vec<Token>) -> Result<Node, SyntaxError> {
    let mut ast = Node::new(NodeKind::ROOT);
    let mut i = 0usize;

    while let Some(tok) = tokens.get(i) {
        match tok.get_kind() {
            EOF => {
                return Ok(ast)
            },
            _ => {
                let stmt = parse_stmt(&tokens, &mut i);
                if stmt.is_err() {
                    return stmt;
                }
                ast.add_node(stmt.unwrap());
            }
        }
        i += 1;
    };

    Ok(ast)
}

fn parse_stmt(tokens: &Vec<Token>, i: &mut usize) -> Result<Node, SyntaxError> {
    match tokens.get(*i).unwrap().get_kind() {
        SEMICOLON => Ok(Node::new(NodeKind::NOOP)),
        DEF => parse_fn(tokens, i),
        ALIAS => parse_alias(tokens, i),
        _ => parse_expr(tokens, i)
    }
}

fn parse_expr(tokens: &Vec<Token>, i: &mut usize) -> Result<Node, SyntaxError> {
    let tok = tokens.get(*i).unwrap();
    match tok.get_kind() {
        LPAREN => {
            *i += 1;
            let expr = parse_expr(tokens, i);
            next_tok!(i, tokens, RPAREN);
            *i += 1;
            expr
        }
        VAR => {
            let mut node = Node::new(NodeKind::VAR);
            node.set_name(tok.get_val().replace("\\", ""));
            Ok(node)
        },
        ID => {
            let mut node = Node::new(NodeKind::CALL);
            node.set_name(tok.get_val().replace("\\", ""));
            Ok(node)
        },
        STRING => {
            let mut node = Node::new(NodeKind::STRING);
            
            node.set_name({
                let mut value = tok.get_val(); // remove the first and last character (the string quotes)
                value.remove(0);
                value.remove(value.len() - 1);
                value
            });

            Ok(node)
        },
        _ => Err(SyntaxError::UnexpectedToken(tok.clone()))
    }
}

fn parse_fn(tokens: &Vec<Token>, i: &mut usize) -> Result<Node, SyntaxError> {
    let mut fn_def = Node::new(NodeKind::FN);
    let tok = next_tok!(i, tokens, ID);
    fn_def.set_name(tok.get_val().replace("\\", ""));

    // parse the function arguments
    if next_is!(i, tokens, LPAREN) {
        *i += 1;

        while let Some(mut tok) = tokens.get(*i) {
            if next_is!(i, tokens, RPAREN) {
                *i += 1;
                break;
            }

            tok = next_tok!(i, tokens, ID);
            fn_def.add_arg({
                let mut arg = Node::new(NodeKind::VAR);
                arg.set_name(tok.get_val());
                arg
            });

            if !next_is!(i, tokens, RPAREN) {
                next_tok!(i, tokens, COMMA);
            }
        }

        *i += 1;
    }

    while tokens.get(*i).is_some() && !next_is!(i, tokens, END) {
        let stmt = parse_stmt(tokens, i);
        if stmt.is_err() {
            return stmt;
        }
        fn_def.add_node(stmt.unwrap());
        *i += 1;
    }

    next_tok!(i, tokens, END);

    Ok(fn_def)
}

fn parse_alias(tokens: &Vec<Token>, i: &mut usize) -> Result<Node, SyntaxError> {
    let mut alias = Node::new(NodeKind::ALIAS);
    alias.set_name(next_tok!(i, tokens, ID).get_val().replace("\\", ""));
    next_tok!(i, tokens, EQUALS);
    *i += 1;
    let value = parse_expr(tokens, i);
    if value.is_err() {
        return value
    }

    alias.add_node(value.unwrap());

    *i += 1;

    Ok(alias)
}