#[test]
fn test_interpret_interpret() {
    use crate::args::Args;
    use crate::ast::ChildNode;
    use crate::interpret::Interpreter;
    use crate::ERROR;

    let interpreter: Interpreter = Interpreter::new();
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

    let interpreter: Interpreter = Interpreter::new();
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

    let interpreter: Interpreter = Interpreter::new();
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
}
