use crate::args;
use crate::ast;
use crate::token;
use crate::ERROR;
use crate::SUCCESS;

pub struct Parser {
    ast_nodes: Vec<ast::ASTNode>,
    current_node: usize,
    tokens: Vec<token::Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            ast_nodes: Vec::new(),
            current_node: 0,
            tokens: Vec::new(),
        }
    }

    // Getter and Setter for ast_nodes
    pub fn get_ast_nodes(&self) -> &Vec<ast::ASTNode> {
        return &self.ast_nodes;
    }

    pub fn set_ast_nodes(&mut self, ast_nodes: Vec<ast::ASTNode>) {
        self.ast_nodes = ast_nodes;
    }

    // Getter and Setter for current_node
    pub fn get_current_node(&self) -> &usize {
        return &self.current_node;
    }

    pub fn set_current_node(&mut self, current_node: usize) {
        self.current_node = current_node;
    }

    // Getter and Setter for tokens
    pub fn get_tokens(&self) -> &Vec<token::Token> {
        return &self.tokens;
    }

    pub fn set_tokens(&mut self, tokens: Vec<token::Token>) {
        self.tokens = tokens;
    }

    // Push function for each vector
    pub fn push_token(&mut self, token: token::Token) {
        self.tokens.push(token);
    }
    pub fn push_ast_node(&mut self, ast_node: ast::ASTNode) {
        self.ast_nodes.push(ast_node);
    }

    // Main parse function
    pub fn parse(&self, _args: &args::Args) -> i32 {
        for token in self.get_tokens() {
            if *token.get_type() == token::TokenType::Error {
                return ERROR;
            }
        }
        return SUCCESS;
    }
}
