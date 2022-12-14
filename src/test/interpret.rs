#[test]
fn test_interpret_interpret() {
    use crate::args::Args;
    use crate::ast::ChildNode;
    use crate::interpret::Interpreter;
    use crate::ERROR;

    let mut interpreter: Interpreter = Interpreter::new();
    let args = Args::create(None, false, false, true, false, false);
    let ast: ChildNode = None;
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);
    let args = Args::create(None, false, false, false, false, false);
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);
}

#[test]
fn test_visit() {
    use crate::args::Args;
    use crate::ast::ASTNode;
    use crate::ast::ASTNodeType;
    use crate::ast::ChildNode;
    use crate::interpret::Interpreter;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut interpreter: Interpreter = Interpreter::new();
    let args = Args::create(None, false, false, true, false, false);
    let mut ast: ChildNode = None;
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        Vec::new(),
        Some(Token::create(TokenType::Number, String::from(""))),
        ASTNodeType::Primative,
    )));

    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        Vec::new(),
        None,
        ASTNodeType::Empty,
    )));

    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    let args = Args::create(None, false, false, false, false, false);
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);
}

#[test]
fn test_interpret_primative() {
    use crate::args::Args;
    use crate::ast::ASTNode;
    use crate::ast::ASTNodeType;
    use crate::ast::ChildNode;
    use crate::interpret::Interpreter;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut interpreter: Interpreter = Interpreter::new();
    let args = Args::create(None, false, false, true, false, false);
    let mut ast: ChildNode;

    ast = Some(Box::new(ASTNode::create(
        Vec::new(),
        Some(Token::create(TokenType::Number, String::from(""))),
        ASTNodeType::Primative,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        Vec::new(),
        None,
        ASTNodeType::Primative,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        Vec::new(),
        Some(Token::create(TokenType::Error, String::from("error"))),
        ASTNodeType::Primative,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        Vec::new(),
        Some(Token::create(TokenType::String, String::from("test"))),
        ASTNodeType::Primative,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        Vec::new(),
        Some(Token::create(TokenType::Boolean, String::from("true"))),
        ASTNodeType::Primative,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        Vec::new(),
        Some(Token::create(TokenType::Identifier, String::from("x"))),
        ASTNodeType::Primative,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::Number, String::from("0"))),
                ASTNodeType::Primative,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::Number, String::from("num"))),
                ASTNodeType::Type,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("x"))),
        ASTNodeType::Declaration,
    )));
    let mut interpreter: Interpreter = Interpreter::new();
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        Vec::new(),
        Some(Token::create(TokenType::Identifier, String::from("x"))),
        ASTNodeType::Primative,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);
    
}

#[test]
fn test_interpret_return() {
    use crate::args::Args;
    use crate::ast::ASTNode;
    use crate::ast::ASTNodeType;
    use crate::ast::ChildNode;
    use crate::interpret::Interpreter;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut interpreter: Interpreter = Interpreter::new();
    let args = Args::create(None, false, false, true, false, false);
    let mut ast: ChildNode;

    ast = Some(Box::new(ASTNode::create(
        vec![None],
        Some(Token::create(TokenType::Return, String::from("return"))),
        ASTNodeType::Return,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![Some(Box::new(ASTNode::create(
            Vec::new(),
            Some(Token::create(TokenType::Number, String::from("0"))),
            ASTNodeType::Primative,
        )))],
        Some(Token::create(TokenType::Return, String::from("return"))),
        ASTNodeType::Return,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);
}

#[test]
fn test_interpret_print() {
    use crate::args::Args;
    use crate::ast::ASTNode;
    use crate::ast::ASTNodeType;
    use crate::ast::ChildNode;
    use crate::interpret::Interpreter;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut interpreter: Interpreter = Interpreter::new();
    let args = Args::create(None, false, false, true, false, false);
    let mut ast: ChildNode;

    ast = Some(Box::new(ASTNode::create(
        vec![None],
        Some(Token::create(TokenType::Return, String::from("return"))),
        ASTNodeType::Print,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![Some(Box::new(ASTNode::create(
            Vec::new(),
            Some(Token::create(TokenType::Number, String::from("0"))),
            ASTNodeType::Primative,
        )))],
        Some(Token::create(TokenType::Print, String::from("print"))),
        ASTNodeType::Print,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);
}

#[test]
fn test_interpret_variable() {
    use crate::args::Args;
    use crate::ast::ASTNode;
    use crate::ast::ASTNodeType;
    use crate::interpret::Interpreter;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let args = Args::create(None, false, false, true, false, false);
    let mut ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::Number, String::from("0"))),
                ASTNodeType::Primative,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::Number, String::from("num"))),
                ASTNodeType::Type,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("x"))),
        ASTNodeType::Declaration,
    )));
    let mut interpreter: Interpreter = Interpreter::new();
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::Number, String::from("0"))),
                ASTNodeType::Primative,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::String, String::from("str"))),
                ASTNodeType::Type,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("x"))),
        ASTNodeType::Declaration,
    )));
    let mut interpreter: Interpreter = Interpreter::new();
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::Number, String::from("0"))),
                ASTNodeType::Primative,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Type,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("x"))),
        ASTNodeType::Declaration,
    )));
    let mut interpreter: Interpreter = Interpreter::new();
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::Number, String::from("0"))),
                ASTNodeType::Primative,
            ))),
            None, 
        ],
        Some(Token::create(TokenType::Identifier, String::from("x"))),
        ASTNodeType::Declaration,
    )));
    let mut interpreter: Interpreter = Interpreter::new();
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![
            None, 
            None, 
        ],
        Some(Token::create(TokenType::Identifier, String::from("x"))),
        ASTNodeType::Declaration,
    )));
    let mut interpreter: Interpreter = Interpreter::new();
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![
            None, 
            None, 
        ],
        Some(Token::create(TokenType::Number, String::from("0"))),
        ASTNodeType::Declaration,
    )));
    let mut interpreter: Interpreter = Interpreter::new();
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![],
        None,
        ASTNodeType::Declaration,
    )));
    let mut interpreter: Interpreter = Interpreter::new();
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);
}

#[test]
fn test_interpret_function() {
    use crate::args::Args;
    use crate::ast::ASTNode;
    use crate::ast::ASTNodeType;
    use crate::interpret::Interpreter;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::SUCCESS;
    use crate::ERROR;

    let args = Args::create(None, false, false, true, false, false);
    let mut ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Parameters,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Block,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("entry"))),
        ASTNodeType::Function,
    )));
    let mut interpreter: Interpreter = Interpreter::new();
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);


    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::Number, String::from("0"))),
                ASTNodeType::Primative,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                Some(Token::create(TokenType::Number, String::from("num"))),
                ASTNodeType::Type,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Declaration,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Parameters,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Block,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Function,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    interpreter = Interpreter::new();

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Parameters,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Block,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Function,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Parameters,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Call,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Parameters,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Block,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Function,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Parameters,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Block,
            ))),
        ],
        None,
        ASTNodeType::Function,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    interpreter = Interpreter::new();

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![
                    Some(Box::new(ASTNode::create(
                        vec![],
                        Some(Token::create(TokenType::Identifier, String::from("x"))),
                        ASTNodeType::Primative,
                    ))),
                ],
                None,
                ASTNodeType::Parameters,
            ))),
            Some(Box::new(ASTNode::create(
                vec![],
                None,
                ASTNodeType::Block,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Function,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![
                    Some(Box::new(ASTNode::create(
                        vec![],
                        Some(Token::create(TokenType::Number, String::from("0"))),
                        ASTNodeType::Primative,
                    ))),
                ],
                None,
                ASTNodeType::Parameters,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Call,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![
                    Some(Box::new(ASTNode::create(
                        vec![],
                        Some(Token::create(TokenType::Number, String::from("0"))),
                        ASTNodeType::Primative,
                    ))),
                ],
                None,
                ASTNodeType::Block,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Call,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![None],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Call,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    ast = Some(Box::new(ASTNode::create(
        vec![],
        None,
        ASTNodeType::Call,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), ERROR);

    interpreter = Interpreter::new();

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![
                    Some(Box::new(ASTNode::create(
                        vec![],
                        Some(Token::create(TokenType::Identifier, String::from("x"))),
                        ASTNodeType::Primative,
                    ))),
                ],
                None,
                ASTNodeType::Parameters,
            ))),
            Some(Box::new(ASTNode::create(
                vec![
                    Some(Box::new(ASTNode::create(
                        vec![],
                        Some(Token::create(TokenType::Error, String::from("error"))),
                        ASTNodeType::Primative,
                    ))),
                ],
                None,
                ASTNodeType::Block,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Function,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);

    ast = Some(Box::new(ASTNode::create(
        vec![
            Some(Box::new(ASTNode::create(
                vec![
                    Some(Box::new(ASTNode::create(
                        vec![],
                        Some(Token::create(TokenType::Number, String::from("0"))),
                        ASTNodeType::Primative,
                    ))),
                ],
                None,
                ASTNodeType::Parameters,
            ))),
        ],
        Some(Token::create(TokenType::Identifier, String::from("test"))),
        ASTNodeType::Call,
    )));
    assert_eq!(interpreter.interpret(&args, &ast), SUCCESS);
}