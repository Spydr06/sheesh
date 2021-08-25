use std::ptr;
use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum ASTNodeKind {
    SCOPE,

    VAR,
    ALIAS,

    FN,
    ARG,

    IF,
    EXIT
}

#[derive(Clone, Debug)]
pub struct ASTNode {
    tok: Token,
    kind: ASTNodeKind,
    callee: String,
    value: *const ASTNode,

    op: char,
    left: *const ASTNode,
    right: *const ASTNode,

    contents: Vec<ASTNode>
}

impl ASTNode {
    pub fn new_scope(contents: Vec<ASTNode>) -> Self {
        Self {
            tok: Token::default(),
            kind: ASTNodeKind::SCOPE,
            callee: String::default(),
            value: ptr::null(),
            op: '\0',
            left: ptr::null(),
            right: ptr::null(),
            contents
        }
    }

    pub fn new_alias(tok: Token, callee: String, value: *const ASTNode) -> Self {
        Self {
            tok,
            kind: ASTNodeKind::ALIAS,
            callee,
            value,
            op: '\0',
            left: ptr::null(),
            right: ptr::null(),
            contents: vec![]
        }
    }

    pub fn new_var(tok: Token, callee: String) -> Self {
        Self {
            tok,
            kind: ASTNodeKind::VAR,
            callee,
            value: ptr::null(),
            op: '\0',
            left: ptr::null(),
            right: ptr::null(),
            contents: vec![]
        }
    }

    pub fn new_exit(tok: Token, exit_code: *const ASTNode) -> Self {
        Self {
            tok,
            kind: ASTNodeKind::EXIT,
            callee: String::default(),
            value: exit_code,
            op: '\0',
            left: ptr::null(),
            right: ptr::null(),
            contents: vec![]
        }
    }

    pub fn get_kind(&self) -> ASTNodeKind {
        self.kind.clone()
    }

    pub fn get_contents(&self) -> Vec<ASTNode> {
        self.contents.clone()
    }
}