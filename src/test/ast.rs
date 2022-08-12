#[test]
fn test_ast_node_types() {
    use crate::ast::ASTNodeType;

    let ast_node_type_empty = ASTNodeType::Empty;
    let ast_node_type_unary = ASTNodeType::Unary;
    let ast_node_type_binary = ASTNodeType::Binary;
    let ast_node_type_ternary = ASTNodeType::Ternary;
    let ast_node_type_quaternary = ASTNodeType::Quaternary;
    let ast_node_type_block = ASTNodeType::Block;
    let ast_node_type_function = ASTNodeType::Function;
    let ast_node_type_parameters = ASTNodeType::Parameters;
    let ast_node_type_declaration = ASTNodeType::Declaration;

    assert_eq!(ast_node_type_empty, ASTNodeType::Empty);
    assert_eq!(ast_node_type_unary, ASTNodeType::Unary);
    assert_eq!(ast_node_type_binary, ASTNodeType::Binary);
    assert_eq!(ast_node_type_ternary, ASTNodeType::Ternary);
    assert_eq!(ast_node_type_quaternary, ASTNodeType::Quaternary);
    assert_eq!(ast_node_type_block, ASTNodeType::Block);
    assert_eq!(ast_node_type_function, ASTNodeType::Function);
    assert_eq!(ast_node_type_parameters, ASTNodeType::Parameters);
    assert_eq!(ast_node_type_declaration, ASTNodeType::Declaration);
}

#[test]
fn test_ast() {
    use crate::ast::ASTNode;
    use crate::ast::ASTNodeType;
    use crate::token::Token;
    use crate::token::TokenType;

    // Construct ASTNode
    let mut ast_node = ASTNode::create(Vec::new(), None, ASTNodeType::Empty);
    let child_node = ASTNode::create(Vec::new(), None, ASTNodeType::Empty);

    // Testing Getters
    assert_eq!(ast_node.get_children().len(), 0);
    assert_eq!(*ast_node.get_operation(), None);
    assert_eq!(*ast_node.get_type(), ASTNodeType::Empty);

    // Testing Setters
    ast_node.set_children(vec![Some(Box::new(child_node))]);
    ast_node.set_operation(Some(Token::create(TokenType::Eof, String::from(""))));
    ast_node.set_type(ASTNodeType::Unary);

    assert_eq!(ast_node.get_children().len(), 1);
    assert_eq!(
        *ast_node.get_operation(),
        Some(Token::create(TokenType::Eof, String::from("")))
    );
    assert_eq!(*ast_node.get_type(), ASTNodeType::Unary);

    // Testing print no value (children Some)
    ast_node.print(0, None);
    // Testing print with value (children Some)
    let file = std::fs::File::create("examples/test/test_ast.dot").expect("Unable to open file");
    ast_node.print(0, Some(file));

    ast_node.set_children(vec![None]);

    // Testing print no value (children None)
    ast_node.print(0, None);
    // Testing print with value (children None)
    let file = std::fs::File::create("examples/test/test_ast.dot").expect("Unable to open file");
    ast_node.print(0, Some(file));

    // Testing Debug for Code Coverage
    println!("{:#?}", ast_node);
}
