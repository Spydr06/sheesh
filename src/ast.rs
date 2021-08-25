use std::ptr;
use crate::lexer::Token;

#[derive(Clone, Debug)]
pub enum ASTNodeKind {
    AST_SCOPE,

    AST_VAR,
    AST_ALIAS,

    AST_FN,
    AST_ARG,

    AST_IF,
    AST_EXIT
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
            kind: ASTNodeKind::AST_SCOPE,
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
            kind: ASTNodeKind::AST_ALIAS,
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
            kind: ASTNodeKind::AST_VAR,
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
            kind: ASTNodeKind::AST_EXIT,
            callee: String::default(),
            value: exit_code,
            op: '\0',
            left: ptr::null(),
            right: ptr::null(),
            contents: vec![]
        }
    }
}