use crate::args;
use crate::ast;
use crate::token;
use crate::ERROR;
use crate::SUCCESS;
pub struct Interpreter {
    variables: std::collections::HashMap<String, Option<token::Token>>,
    functions: std::collections::HashMap<String, Vec<ast::ChildNode>>,
}
impl Interpreter {
    pub fn new() -> Self {
        return Interpreter {
            variables: std::collections::HashMap::new(),
            functions: std::collections::HashMap::new(),
        };
    }

    fn visit_primative(&self, ast_ptr: &ast::ASTNode) -> Option<token::Token> {
            match ast_ptr.get_operation() {
                Some(operation) => match operation.get_type() {
                    token::TokenType::Number => return ast_ptr.get_operation().clone(),
                    token::TokenType::String => return ast_ptr.get_operation().clone(),
                    token::TokenType::Boolean => return ast_ptr.get_operation().clone(),
                    _ => return None,
                },
                None => return None,
            }
    }

    fn visit(&self, ast: &ast::ChildNode) -> Option<token::Token> {
        match ast {
            Some(ast_ptr) => match ast_ptr.get_type() {
                ast::ASTNodeType::Primative => return self.visit_primative(ast_ptr),
                _ => {
                    println!("TODO: INTERPRETER -> visit() Some value in match, ASTNodeType not Covered");
                    return None;
                }
            }
            None => {
                println!("TODO: INTERPRETER -> visit() None value in match");
                return None;
            }
        }
    }

    pub fn interpret(&self, args: &args::Args, ast: &ast::ChildNode) -> i32 {
        if *args.get_interpret() {
            match self.visit(ast) {
                Some(_) => return SUCCESS,
                None => return ERROR,
            }
        }
        return ERROR;
    }
}
