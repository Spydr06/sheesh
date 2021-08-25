use crate::ast::{ASTNode, ASTNodeKind};
use std::process::exit;

pub fn evaluate(ast: ASTNode) -> Result<i32, &'static str> {
    if ast.get_kind() == ASTNodeKind::SCOPE {
        eval_scope(ast)
    } else {
        Err("Cannot evaluate anything other than a Scope")
    }
}

fn eval_scope(ast: ASTNode) -> Result<i32, &'static str> {
    if ast.get_kind() != ASTNodeKind::SCOPE {
        Err("Expected Scope")
    } else {
        for stmt in ast.get_contents() {
            match stmt.get_kind() {
                ASTNodeKind::EXIT => {
                    exit(0)
                }
        
                _ => {
                    return Err("Expected Statement");
                }
            }
        }

        Ok(0)
    }
}