use std::ptr;

use crate::ast::ASTNode;
use crate::lexer::{Token, TokenKind};

struct ParserData {
    pub tokens: Vec<Token>,
    pub cur_tok: Token,
    pub tok_idx: usize
}

impl ParserData {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.clone(),
            cur_tok: tokens.get(0).unwrap().to_owned().clone(),
            tok_idx: 0
        }
    }

    pub fn next(&mut self) -> Token {
        self.tok_idx += 1;
        self.cur_tok = self.tokens.get(self.tok_idx).unwrap().to_owned();
        self.cur_tok.clone()
    }

    pub fn cur(&self) -> Token {
        self.cur_tok.clone()
    }

    pub fn tok_is(&self, kind:TokenKind) -> bool {
        self.cur_tok.get_kind() == kind
    }
}

pub fn parse_ast(tokens: Vec<Token>) -> ASTNode {
    let mut parser = ParserData::new(tokens);
    
    parse_scope(&mut parser)
}

fn parse_scope(parser: &mut ParserData) -> ASTNode {
    let mut contents: Vec<ASTNode> = vec![];

    while parser.cur().get_kind() != TokenKind::EOF && parser.cur().get_kind() != TokenKind::END {
        contents.push(
            match parser.cur().get_kind() {
                EXIT => {
                    parse_exit(parser)
                }
            }
        );
    }
    if parser.tok_is(TokenKind::END) {
        parser.next();
    }
    
    ASTNode::new_scope(contents)
}

fn parse_exit(parser: &mut ParserData) -> ASTNode {
    let exit_tok = parser.cur();
    parser.next();

    ASTNode::new_exit(exit_tok, ptr::null())
}