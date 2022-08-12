use crate::token;
use std::fs::File;
use std::io::Write;

pub type ChildNode = Option<Box<ASTNode>>;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum ASTNodeType {
    Empty,
    Primative,
    Return,
    Unary,
    Binary,
    Ternary,
    Quaternary,
    Block,
    Function,
    Parameters,
    Declaration,
}

#[derive(Clone, Debug)]
pub struct ASTNode {
    children: Vec<ChildNode>,
    operation: Option<token::Token>,
    node_type: ASTNodeType,
}

#[allow(dead_code)]
impl ASTNode {
    pub fn print(&self, layer: usize, file: Option<File>) {
        match file {
            Some(mut output_file) => {
                write!(output_file, "Node{}", layer).expect("Unable to write to file");
                for i in 0..self.children.len() {
                    match &self.children[i] {
                        Some(child) => {
                            writeln!(output_file, " -> Node{}", layer)
                                .expect("Unable to write to file");
                            child.print(
                                layer + 1,
                                Some(
                                    output_file
                                        .try_clone()
                                        .expect("Unable to clone file: examples/ast.dot {e:#?}"),
                                ),
                            )
                        }
                        None => {
                            println!("TODO: Write Error Message");
                        }
                    }
                }
            }
            None => {
                for i in 0..self.children.len() {
                    match &self.children[i] {
                        Some(child) => child.print(layer + 1, None),
                        None => {
                            println!("TODO: Write Error Message");
                        }
                    }
                }
                for _ in 0..layer {
                    print!("\t");
                }
                println!("{:?}", self.operation);
            }
        }
    }

    pub fn create(
        children: Vec<ChildNode>,
        operation: Option<token::Token>,
        node_type: ASTNodeType,
    ) -> Self {
        ASTNode {
            children: children,
            operation: operation,
            node_type: node_type,
        }
    }

    // Getter and Setter for children
    pub fn get_children(&self) -> &Vec<ChildNode> {
        return &self.children;
    }

    pub fn set_children(&mut self, children: Vec<ChildNode>) {
        self.children = children;
    }

    // Getter and Setter for operation
    pub fn get_operation(&self) -> &Option<token::Token> {
        return &self.operation;
    }

    pub fn set_operation(&mut self, operation: Option<token::Token>) {
        self.operation = operation;
    }

    // Getter and Setter for node_type
    pub fn get_type(&self) -> &ASTNodeType {
        return &self.node_type;
    }

    pub fn set_type(&mut self, node_type: ASTNodeType) {
        self.node_type = node_type;
    }
}
