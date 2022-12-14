use crate::args;
use crate::ast;
use crate::token;
use crate::ERROR;
use crate::SUCCESS;

#[allow(dead_code)]
pub struct Interpreter {
    variables: std::collections::HashMap<String, Option<token::Token>>,
    functions: std::collections::HashMap<String, Vec<ast::ChildNode>>,
}
#[allow(dead_code)]
impl Interpreter {
    pub fn new() -> Self {
        return Interpreter {
            variables: std::collections::HashMap::new(),
            functions: std::collections::HashMap::new(),
        };
    }

    pub fn get_functions(&self) -> &std::collections::HashMap<String, Vec<ast::ChildNode>> {
        return &self.functions;
    }

    fn visit_primative(&mut self, ast_ptr: &ast::ASTNode) -> Option<token::Token> {
        match ast_ptr.get_operation() {
            Some(operation) => match operation.get_type() {
                token::TokenType::Number => return ast_ptr.get_operation().clone(),
                token::TokenType::String => return ast_ptr.get_operation().clone(),
                token::TokenType::Boolean => return ast_ptr.get_operation().clone(),
                token::TokenType::Identifier => match self.variables.get(operation.get_data()){
                    Some(value) => {
                        return value.clone();
                    },
                    None => {
                        println!("TODO: INTERPRETER -> visit_declaration() None value");
                        return None;
                    },
                },
                _ => return None,
            },
            None => return None,
        }
    }

    fn visit_return(&mut self, ast_ptr: &ast::ASTNode) -> Option<token::Token> {
        return self.visit(&ast_ptr.get_children()[0]);
    }

    fn visit_print(&mut self, ast_ptr: &ast::ASTNode) -> Option<token::Token> {
        match self.visit(&ast_ptr.get_children()[0]) {
            Some(value) => {
                println!("{}", value.get_data());
                return ast_ptr.get_operation().clone();
            }
            None => return None,
        }
    }

    fn visit_declaration(&mut self, ast_ptr: &ast::ASTNode) -> Option<token::Token> {
        match ast_ptr.get_operation() {
            Some(operation) => match operation.get_type() {
                token::TokenType::Identifier => match self.visit(&ast_ptr.get_children()[0]) {
                    Some(value) => match &ast_ptr.get_children()[1] {
                        Some(value_type) => match &value_type.get_operation() {
                            Some(value_type_operation) => {
                                if value.get_type() == value_type_operation.get_type() {
                                    self.variables
                                        .insert(operation.get_data().to_string(), Some(value));
                                    return self.visit(&ast_ptr.get_children()[0]);
                                } else {
                                    eprintln!(
                                        "Mismatched datatypes, expecter {:?}, found {:?}",
                                        value_type_operation.get_type(),
                                        value.get_type()
                                    );
                                    return None;
                                }
                            }
                            None => {
                                self.variables
                                    .insert(operation.get_data().to_string(), Some(value));
                                return self.visit(&ast_ptr.get_children()[0]);
                            }
                        },
                        None => {
                            println!("TODO: INTERPRETER -> visit_declaration() None value in child_1, Identifier");
                            return None;
                        }
                    },
                    None => {
                        println!("TODO: INTERPRETER -> visit_declaration() None value in child_0, Identifier");
                        return None;
                    }
                },
                _ => {
                    println!("TODO: INTERPRETER -> visit_declaration() Some value in operation, Not Identifier");
                    return None;
                }
            },
            None => {
                println!("TODO: INTERPRETER -> visit_declaration() None value in operation");
                return None;
            }
        }
    }

    fn visit_function(&mut self, ast_ptr: &ast::ASTNode) -> Option<token::Token> {
        match ast_ptr.get_operation() {
            Some(operation) => match self.variables.get(operation.get_data()) {
                Some(_) => return None,
                None => match self.functions.get(operation.get_data()){
                    Some(_) => return None,
                    None => {
                        let name = operation.get_data();
                        self.functions.insert(name.to_string(), ast_ptr.get_children().to_vec());
                        return Some(operation.clone());
                    },
                }
            },
            None => return None,
        }
    }

    fn visit_call(&mut self, ast_ptr: &ast::ASTNode) -> Option<token::Token> {
        match ast_ptr.get_operation() {
            Some(operation) => match &ast_ptr.get_children()[0] {
                Some(parameters_child) => match parameters_child.get_type() {
                    ast::ASTNodeType::Parameters => {
                        let parameters = self.functions.get(operation.get_data()).unwrap()[0].clone().unwrap().get_children().clone();
                        let previous_variables = self.variables.clone();
                        for param_num in 0..parameters.len() {
                            let value = self.visit(
                                &ast_ptr.get_children()[0].clone().unwrap().get_children()[param_num],
                            );
                            self.variables.insert(
                                parameters[param_num].clone().unwrap().get_operation().clone().unwrap().get_data().to_string(),
                                value,
                            );
                        }
                        let tok = self.visit(&mut self.functions.get(operation.get_data()).unwrap()[1].clone());
                        self.variables = previous_variables.clone();
                        return tok;
                    },
                    _ => {
                        let value = self.visit(&ast_ptr.get_children()[0]);
                        self.variables.insert(operation.get_data().to_string(), value);
                        return None;
                    }
                },
                None => return None,
            },
            None => return None,
        }
    }

    fn visit_block(&mut self, ast_ptr: &ast::ASTNode) -> Option<token::Token> {
        let mut previous_variables = self.variables.clone();
        let mut return_value: Option<token::Token> = Some(token::Token::create(
            token::TokenType::Nil,
            String::from(""),
        ));

        for mut child in ast_ptr.get_children() {
            match self.visit(&mut child) {
                Some(value) => {
                    return_value = Some(value);
                    break;
                }
                None => {},
            }
        }

        for (key, value) in self.variables.iter() {
            if previous_variables.contains_key(key) {
                previous_variables.insert(key.clone(), value.clone());
            }
        }

        self.variables = previous_variables.clone();
        return return_value;
    }


    fn visit(&mut self, ast: &ast::ChildNode) -> Option<token::Token> {
        match ast {
            Some(ast_ptr) => {
                match ast_ptr.get_type() {
                    ast::ASTNodeType::Primative => return self.visit_primative(ast_ptr),
                    ast::ASTNodeType::Return => return self.visit_return(ast_ptr),
                    ast::ASTNodeType::Print => return self.visit_print(ast_ptr),
                    ast::ASTNodeType::Declaration => return self.visit_declaration(ast_ptr),
                    ast::ASTNodeType::Function => return self.visit_function(ast_ptr),
                    ast::ASTNodeType::Call => return self.visit_call(ast_ptr),
                    ast::ASTNodeType::Block => return self.visit_block(ast_ptr),
                    _ => {
                        println!("TODO: INTERPRETER -> visit() Some value in match, ASTNodeType not Covered");
                        return None;
                    }
                }
            }
            None => return None,
        }
    }

    pub fn interpret(&mut self, args: &args::Args, ast: &ast::ChildNode) -> i32 {
        if *args.get_interpret() {
            match self.visit(ast) {
                Some(_) => return SUCCESS,
                None => return ERROR,
            }
        }
        return ERROR;
    }
}
