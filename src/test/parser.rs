#[test]
fn test_parse() {
    use crate::args::Args;
    use crate::ast::ASTNode;
    use crate::ast::ASTNodeType;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    // Construct Parser
    let mut parser: Parser = Parser::new();
    let ast_node = ASTNode::create(Vec::new(), None, ASTNodeType::Empty);
    let ast_node_push = ASTNode::create(Vec::new(), None, ASTNodeType::Empty);
    let args = Args::create(None, false, false, false, false, false);

    // Test Getters
    assert_eq!(parser.get_ast_nodes().len(), 0);
    assert_eq!(*parser.get_current_node(), 0);
    assert_eq!(parser.get_tokens().len(), 0);

    parser.set_ast_nodes(vec![ast_node]);
    parser.set_current_node(1);
    parser.set_tokens(vec![Token::create(TokenType::Eof, String::from(""))]);

    // Test Setters
    assert_eq!(parser.get_ast_nodes().len(), 1);
    assert_eq!(*parser.get_current_node(), 1);
    assert_eq!(parser.get_tokens().len(), 1);

    // Test Push Functions
    parser.push_ast_node(ast_node_push);
    parser.push_token(Token::create(TokenType::Eof, String::from("")));

    assert_eq!(parser.get_ast_nodes().len(), 2);
    assert_eq!(parser.get_tokens().len(), 2);

    // Test Parse Function
    assert_eq!(parser.parse(&args), SUCCESS);
    parser.push_token(Token::create(TokenType::Error, String::from("error")));
    assert_eq!(parser.parse(&args), ERROR);

    parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("0")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_block() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Number, String::from("0")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_declaration() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Funct, String::from("funct")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    parser = Parser::new();
    parser.push_token(Token::create(TokenType::Identifier, String::from("asdf")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    parser = Parser::new();
    parser.push_token(Token::create(TokenType::Identifier, String::from("asdf")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("0")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_function_declaration() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Funct, String::from("funct")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Funct, String::from("funct")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("a")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Funct, String::from("funct")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Funct, String::from("funct")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_var_declaration() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Num, String::from("num")));
    parser.push_token(Token::create(TokenType::Equal, String::from("=")));
    parser.push_token(Token::create(TokenType::Number, String::from("0")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Str, String::from("str")));
    parser.push_token(Token::create(TokenType::Equal, String::from("=")));
    parser.push_token(Token::create(TokenType::String, String::from("test")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);
}

#[test]
fn test_parse_statement() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Addition, String::from("+")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Print, String::from("print")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Return, String::from("return")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::If, String::from("if")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::While, String::from("while")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::For, String::from("for")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::While, String::from("while")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::While, String::from("while")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::Return, String::from("return")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Number, String::from("0")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);
}

#[test]
fn test_parse_expression_statement() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Identifier, String::from("x")));
    parser.push_token(Token::create(TokenType::Equal, String::from("=")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Identifier, String::from("x")));
    parser.push_token(Token::create(TokenType::Equal, String::from("=")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Identifier, String::from("x")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Addition, String::from("+")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Addition, String::from("+")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);
}

#[test]
fn test_parse_for_statement() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::For, String::from("for")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::For, String::from("for")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::For, String::from("for")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::For, String::from("for")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::For, String::from("for")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("i")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);
}

#[test]
fn test_parse_while_statement() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::While, String::from("while")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::While, String::from("while")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_if_statement() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::If, String::from("if")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::If, String::from("if")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Else, String::from("else")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::If, String::from("if")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Else, String::from("else")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::If, String::from("if")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Else, String::from("else")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::If, String::from("if")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Else, String::from("else")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::If, String::from("if")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Else, String::from("else")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::If, String::from("if")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::CloseBrace, String::from("}")));
    parser.push_token(Token::create(TokenType::Else, String::from("else")));
    parser.push_token(Token::create(TokenType::OpenBrace, String::from("{")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_return_statement() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Return, String::from("return")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Return, String::from("return")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Number, String::from("0")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Return, String::from("return")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Number, String::from("0")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Return, String::from("return")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_print_statement() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Print, String::from("print")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Print, String::from("print")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);
}

#[test]
fn test_parse_expression() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Print, String::from("print")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Identifier, String::from("x")));
    parser.push_token(Token::create(TokenType::Equal, String::from("=")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Identifier, String::from("x")));
    parser.push_token(Token::create(TokenType::Equal, String::from("=")));
    parser.push_token(Token::create(TokenType::Not, String::from("!")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("x")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Identifier, String::from("x")));
    parser.push_token(Token::create(TokenType::Equal, String::from("=")));
    parser.push_token(Token::create(TokenType::Not, String::from("!")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_terminal() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Addition, String::from("+")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Subtract, String::from("-")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Multiply, String::from("*")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Divide, String::from("/")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Less, String::from("<")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Greater, String::from(">")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::LessEqual, String::from("<=")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::GreaterEqual, String::from(">=")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::BangEqual, String::from("!=")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::EqualEqual, String::from("==")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::EqualEqual, String::from("==")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::EqualEqual, String::from("==")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_primary() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Boolean, String::from("true")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::String, String::from("test")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Addition, String::from("+")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Addition, String::from("+")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Addition, String::from("+")));
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}

#[test]
fn test_parse_primary_block() {
    use crate::args::Args;
    use crate::parse::Parser;
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::ERROR;
    use crate::SUCCESS;

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::Comma, String::from(",")));
    parser.push_token(Token::create(TokenType::Number, String::from("1")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), SUCCESS);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Comma, String::from(",")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::CloseParen, String::from(")")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);

    let mut parser: Parser = Parser::new();
    let args = Args::create(None, false, false, false, false, false);
    parser.push_token(Token::create(TokenType::Identifier, String::from("test")));
    parser.push_token(Token::create(TokenType::OpenParen, String::from("(")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Comma, String::from(",")));
    parser.push_token(Token::create(TokenType::Colon, String::from(":")));
    parser.push_token(Token::create(TokenType::Semicolon, String::from(";")));
    parser.push_token(Token::create(TokenType::Eof, String::from("")));
    assert_eq!(parser.parse(&args), ERROR);
}
