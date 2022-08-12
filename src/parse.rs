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

#[allow(dead_code)]
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

    fn increment(&mut self) {
        if *self.tokens[self.current_node].get_type() != token::TokenType::Eof {
            self.current_node = self.current_node + 1;
        }
    }
    fn decrement(&mut self) {
        self.current_node = self.current_node - 1;
    }

    fn consume(&mut self, token_types: Vec<token::TokenType>) -> Option<token::Token> {
        let mut result = Some(self.tokens[self.current_node].clone());
        for token_type in token_types {
            if *self.tokens[self.current_node].get_type() == token_type {
                self.increment();
            } else {
                eprintln!(
                    "Expected: {:?}, Found: {:?}",
                    token_type,
                    self.tokens[self.current_node].get_type()
                );
                result = None;
            }
        }
        return result;
    }

    fn number(&mut self) -> Option<ast::ASTNode> {
        self.consume(vec![token::TokenType::Number]);
        return Some(ast::ASTNode::create(
            Vec::new(),
            Some(self.tokens[self.current_node - 1].clone()),
            ast::ASTNodeType::Empty,
        ));
    }

    fn bool(&mut self) -> Option<ast::ASTNode> {
        self.consume(vec![token::TokenType::Bool]);
        return Some(ast::ASTNode::create(
            Vec::new(),
            Some(self.tokens[self.current_node - 1].clone()),
            ast::ASTNodeType::Empty,
        ));
    }

    fn string(&mut self) -> Option<ast::ASTNode> {
        self.consume(vec![token::TokenType::String]);
        return Some(ast::ASTNode::create(
            Vec::new(),
            Some(self.tokens[self.current_node - 1].clone()),
            ast::ASTNodeType::Empty,
        ));
    }

    fn identifier(&mut self) -> Option<ast::ASTNode> {
        self.consume(vec![token::TokenType::Identifier]);
        if self.match_tokens(vec![token::TokenType::OpenParen]) {
            match self.primary() {
                Some(primary) => {
                    return Some(ast::ASTNode::create(
                        vec![Some(Box::new(primary))],
                        Some(self.tokens[self.current_node - 1].clone()),
                        ast::ASTNodeType::Empty,
                    ));
                }
                None => return None,
            }
        }
        return Some(ast::ASTNode::create(
            Vec::new(),
            Some(self.tokens[self.current_node - 1].clone()),
            ast::ASTNodeType::Empty,
        ));
    }

    fn primary_block(&mut self) -> Option<ast::ASTNode> {
        self.consume(vec![token::TokenType::OpenParen]);
        let mut parameters: Vec<ast::ChildNode> = Vec::new();
        while !self.match_tokens(vec![token::TokenType::CloseParen]) {
            if *self.tokens[self.current_node + 1].get_type() == token::TokenType::Comma {
                match self.expression() {
                    Some(expression) => {
                        parameters.push(Some(Box::new(expression)));
                        self.consume(vec![token::TokenType::Comma]);
                    }
                    None => return None,
                }
            } else {
                match self.expression() {
                    Some(value) => parameters.push(Some(Box::new(value))),
                    None => return None,
                }
            }
        }
        self.consume(vec![token::TokenType::CloseParen]);
        return Some(ast::ASTNode::create(
            parameters,
            None,
            ast::ASTNodeType::Parameters,
        ));
    }

    fn primary(&mut self) -> Option<ast::ASTNode> {
        return match self.tokens[self.current_node].get_type() {
            token::TokenType::Number => self.number(),
            token::TokenType::Bool => self.bool(),
            token::TokenType::String => self.string(),
            token::TokenType::OpenParen => self.primary_block(),
            token::TokenType::Identifier => self.identifier(),
            _ => {
                eprintln!(
                    "Not Yet Implemented: {:?}",
                    self.consume(vec![self.tokens[self.current_node].get_type().clone()])
                );
                return None;
            }
        };
    }

    fn terminal(&mut self) -> Option<ast::ASTNode> {
        let mut node_ptr = self.primary();
        loop {
            if self.match_tokens(vec![
                token::TokenType::Addition,
                token::TokenType::Subtract,
                token::TokenType::Multiply,
                token::TokenType::Divide,
                token::TokenType::EqualEqual,
                token::TokenType::BangEqual,
                token::TokenType::LessEqual,
                token::TokenType::GreaterEqual,
                token::TokenType::Less,
                token::TokenType::Greater,
            ]) {
                let token = self.consume(vec![self.tokens[self.current_node].get_type().clone()]);
                match self.primary() {
                    Some(primary) => match node_ptr {
                        Some(node) => {
                            node_ptr = Some(ast::ASTNode::create(
                                vec![Some(Box::new(node)), Some(Box::new(primary))],
                                token,
                                ast::ASTNodeType::Binary,
                            ));
                        }
                        None => return None,
                    },
                    None => return None,
                }
            } else {
                break;
            }
        }
        return node_ptr;
    }

    fn expression(&mut self) -> Option<ast::ASTNode> {
        if self.match_tokens(vec![token::TokenType::Equal]) {
            self.consume(vec![token::TokenType::Equal]);
            return self.expression();
        } else if self.match_tokens(vec![token::TokenType::Not]) {
            let not = self.consume(vec![token::TokenType::Not]);
            match self.expression() {
                Some(expression) => {
                    return Some(ast::ASTNode::create(
                        vec![Some(Box::new(expression))],
                        not,
                        ast::ASTNodeType::Unary,
                    ));
                }
                None => return None,
            }
        }
        return self.terminal();
    }

    fn print_statement(&mut self) -> Option<ast::ASTNode> {
        let token = self.tokens[self.current_node].clone();
        if self.match_tokens(vec![token::TokenType::Print]) {
            self.consume(vec![token::TokenType::Print, token::TokenType::OpenParen]);
        }
        let mut node: Option<ast::ASTNode> = None;
        if !self.match_tokens(vec![token::TokenType::CloseParen]) {
            node = self.expression();
        }
        self.consume(vec![
            token::TokenType::CloseParen,
            token::TokenType::Semicolon,
        ]);
        match node {
            Some(node_ptr) => {
                return Some(ast::ASTNode::create(
                    vec![Some(Box::new(node_ptr))],
                    Some(token),
                    ast::ASTNodeType::Unary,
                ))
            }
            None => return None,
        }
    }

    fn return_statement(&mut self) -> Option<ast::ASTNode> {
        let token = self.tokens[self.current_node].clone();
        match self.consume(vec![token::TokenType::Return, token::TokenType::OpenParen]) {
            Some(_) => match self.expression() {
                Some(node) => match self.consume(vec![
                    token::TokenType::CloseParen,
                    token::TokenType::Semicolon,
                ]) {
                    Some(_) => {
                        return Some(ast::ASTNode::create(
                            vec![Some(Box::new(node))],
                            Some(token),
                            ast::ASTNodeType::Unary,
                        ))
                    }
                    None => return None,
                },
                None => return None,
            },
            None => return None,
        }
    }

    fn if_statement(&mut self) -> Option<ast::ASTNode> {
        let token = self.tokens[self.current_node].clone();
        self.consume(vec![token::TokenType::If]);
        let condition = self.expression();
        let then_branch = self.statement();

        match condition {
            Some(condition_ptr) => match then_branch {
                Some(then_branch_ptr) => {
                    if self.match_tokens(vec![token::TokenType::Else]) {
                        self.consume(vec![token::TokenType::Else]);
                        match self.statement() {
                            Some(else_branch_ptr) => {
                                return Some(ast::ASTNode::create(
                                    vec![
                                        Some(Box::new(condition_ptr)),
                                        Some(Box::new(then_branch_ptr)),
                                        Some(Box::new(else_branch_ptr)),
                                    ],
                                    Some(token),
                                    ast::ASTNodeType::Ternary,
                                ));
                            }
                            None => return None,
                        }
                    }
                    return Some(ast::ASTNode::create(
                        vec![
                            Some(Box::new(condition_ptr)),
                            Some(Box::new(then_branch_ptr)),
                            None,
                        ],
                        Some(token),
                        ast::ASTNodeType::Ternary,
                    ));
                }
                None => return None,
            },
            None => return None,
        }
    }

    fn while_statement(&mut self) -> Option<ast::ASTNode> {
        let token = self.tokens[self.current_node].clone();
        self.consume(vec![token::TokenType::While]);
        let condition = self.expression();
        let body = self.statement();
        match condition {
            Some(condition_ptr) => match body {
                Some(body_ptr) => {
                    return Some(ast::ASTNode::create(
                        vec![Some(Box::new(condition_ptr)), Some(Box::new(body_ptr))],
                        Some(token),
                        ast::ASTNodeType::Binary,
                    ));
                }
                None => return None,
            },
            None => return None,
        }
    }

    fn for_statement(&mut self) -> Option<ast::ASTNode> {
        let token = self.tokens[self.current_node].clone();
        self.consume(vec![token::TokenType::For, token::TokenType::OpenParen]);
        let init = self.declaration();
        let condition = self.expression();
        self.consume(vec![token::TokenType::Semicolon]);
        let increment = self.statement();
        self.consume(vec![token::TokenType::CloseParen]);
        let body = self.statement();
        match init {
            Some(init_ptr) => match condition {
                Some(condition_ptr) => match increment {
                    Some(increment_ptr) => match body {
                        Some(body_ptr) => {
                            return Some(ast::ASTNode::create(
                                vec![
                                    Some(Box::new(init_ptr)),
                                    Some(Box::new(condition_ptr)),
                                    Some(Box::new(increment_ptr)),
                                    Some(Box::new(body_ptr)),
                                ],
                                Some(token),
                                ast::ASTNodeType::Quaternary,
                            ));
                        }
                        None => return None,
                    },
                    None => return None,
                },
                None => return None,
            },
            None => return None,
        }
    }

    fn expression_statement(&mut self) -> Option<ast::ASTNode> {
        let node: Option<ast::ASTNode>;
        if self.match_tokens(vec![token::TokenType::Identifier]) {
            let name = self.consume(vec![token::TokenType::Identifier]);
            if !self.match_tokens(vec![token::TokenType::Semicolon]) {
                match self.expression() {
                    Some(expression) => {
                        if !self.match_tokens(vec![token::TokenType::CloseParen]) {
                            self.consume(vec![token::TokenType::Semicolon]);
                        }
                        return Some(ast::ASTNode::create(
                            vec![Some(Box::new(expression))],
                            name,
                            ast::ASTNodeType::Unary,
                        ));
                    }
                    None => return None,
                }
            }
            self.consume(vec![token::TokenType::Semicolon]);
            return Some(ast::ASTNode::create(
                Vec::new(),
                name,
                ast::ASTNodeType::Empty,
            ));
        }
        node = self.expression();
        match self.consume(vec![token::TokenType::Semicolon]) {
            Some(_) => return node,
            None => return None,
        }
    }

    fn statement(&mut self) -> Option<ast::ASTNode> {
        if self.match_tokens(vec![token::TokenType::Print]) {
            return self.print_statement();
        } else if self.match_tokens(vec![token::TokenType::Return]) {
            return self.return_statement();
        } else if self.match_tokens(vec![token::TokenType::If]) {
            return self.if_statement();
        } else if self.match_tokens(vec![token::TokenType::While]) {
            return self.while_statement();
        } else if self.match_tokens(vec![token::TokenType::For]) {
            return self.for_statement();
        } else if self.match_tokens(vec![token::TokenType::OpenBrace]) {
            self.consume(vec![token::TokenType::OpenBrace]);
            let mut statements = Vec::new();
            while !self.match_tokens(vec![token::TokenType::CloseBrace]) {
                match self.block() {
                    Some(block) => statements.push(Some(Box::new(block))),
                    None => return None,
                }
            }
            self.consume(vec![token::TokenType::CloseBrace]);
            return Some(ast::ASTNode::create(
                statements,
                Some(token::Token::create(token::TokenType::Block, String::new())),
                ast::ASTNodeType::Block,
            ));
        }
        return self.expression_statement();
    }

    fn var_declaration(&mut self) -> Option<ast::ASTNode> {
        let token: Option<token::Token>;
        let identifier = self.tokens[self.current_node].clone();
        self.consume(vec![token::TokenType::Identifier]);
        self.consume(vec![token::TokenType::Colon]);
        match self.tokens[self.current_node].get_type() {
            token::TokenType::Num => {
                self.consume(vec![token::TokenType::Num]);
                token = Some(token::Token::create(
                    token::TokenType::Number,
                    String::new(),
                ));
            }
            token::TokenType::Str => {
                self.consume(vec![token::TokenType::Str]);
                token = Some(token::Token::create(
                    token::TokenType::String,
                    String::new(),
                ));
            }
            _ => {
                token = None;
            }
        }
        let mut node: Option<ast::ASTNode> = None;
        if self.match_tokens(vec![token::TokenType::Equal]) {
            node = self.expression();
        }
        self.consume(vec![token::TokenType::Semicolon]);
        match node {
            Some(node_ptr) => {
                return Some(ast::ASTNode::create(
                    vec![
                        Some(Box::new(node_ptr)),
                        Some(Box::new(ast::ASTNode::create(
                            Vec::new(),
                            token,
                            ast::ASTNodeType::Empty,
                        ))),
                    ],
                    Some(identifier),
                    ast::ASTNodeType::Declaration,
                ));
            }
            None => return None,
        }
    }

    fn function_declaration(&mut self) -> Option<ast::ASTNode> {
        self.consume(vec![token::TokenType::Funct]);
        let identifier = self.tokens[self.current_node].clone();
        match self.consume(vec![token::TokenType::Identifier]) {
            Some(_) => {
                let mut parameters = Vec::new();
                match self.expression() {
                    Some(expression) => parameters.push(Some(Box::new(expression))),
                    None => return None,
                }
                match self.block() {
                    Some(block) => parameters.push(Some(Box::new(block))),
                    None => return None,
                }
                return Some(ast::ASTNode::create(
                    parameters,
                    Some(identifier),
                    ast::ASTNodeType::Function,
                ));
            }
            None => return None,
        }
    }

    fn declaration(&mut self) -> Option<ast::ASTNode> {
        if self.match_tokens(vec![token::TokenType::Identifier]) {
            self.increment();
            if self.match_tokens(vec![token::TokenType::Colon]) {
                self.decrement();
                return self.var_declaration();
            }
            self.decrement();
        } else if self.match_tokens(vec![token::TokenType::Funct]) {
            return self.function_declaration();
        }
        return self.statement();
    }

    fn block(&mut self) -> Option<ast::ASTNode> {
        return self.declaration();
    }

    fn match_tokens(&self, token_types: Vec<token::TokenType>) -> bool {
        for token_type in token_types {
            if *self.tokens[self.current_node].get_type() == token_type {
                return true;
            }
        }
        return false;
    }

    // Main parse function
    pub fn parse(&mut self, _args: &args::Args) -> i32 {
        let mut has_error: bool = false;
        while !self.match_tokens(vec![token::TokenType::Eof]) {
            match self.block() {
                Some(node) => self.push_ast_node(node),
                None => {
                    has_error = true;
                }
            }
        }
        for token in self.get_tokens() {
            if *token.get_type() == token::TokenType::Error {
                return ERROR;
            }
        }
        if has_error {
            return ERROR;
        }
        return SUCCESS;
    }
}
