#[test]
fn test_lexer() {
    use crate::lex::Lexer;
    use crate::token::Token;
    use crate::token::TokenType;

    let contents = String::from(
        "(){},.:;
            != ! == = <= < >= >
            + - * /
            \"test\"
            10
            if else
            true false
            while for
            funct num str
            print return
            test",
    );
    let lexer = Lexer::from_iter(contents.chars());
    let mut tokens = Vec::new();

    for token in lexer {
        tokens.push(token);
    }

    assert_eq!(
        vec![
            Token::create(TokenType::OpenParen, String::from("(")),
            Token::create(TokenType::CloseParen, String::from(")")),
            Token::create(TokenType::OpenBrace, String::from("{")),
            Token::create(TokenType::CloseBrace, String::from("}")),
            Token::create(TokenType::Comma, String::from(",")),
            Token::create(TokenType::Dot, String::from(".")),
            Token::create(TokenType::Colon, String::from(":")),
            Token::create(TokenType::Semicolon, String::from(";")),
            Token::create(TokenType::BangEqual, String::from("!=")),
            Token::create(TokenType::Not, String::from("!")),
            Token::create(TokenType::EqualEqual, String::from("==")),
            Token::create(TokenType::Equal, String::from("=")),
            Token::create(TokenType::LessEqual, String::from("<=")),
            Token::create(TokenType::Less, String::from("<")),
            Token::create(TokenType::GreaterEqual, String::from(">=")),
            Token::create(TokenType::Greater, String::from(">")),
            Token::create(TokenType::Addition, String::from("+")),
            Token::create(TokenType::Subtract, String::from("-")),
            Token::create(TokenType::Multiply, String::from("*")),
            Token::create(TokenType::Divide, String::from("/")),
            Token::create(TokenType::String, String::from("test")),
            Token::create(TokenType::Number, String::from("10")),
            Token::create(TokenType::If, String::from("if")),
            Token::create(TokenType::Else, String::from("else")),
            Token::create(TokenType::Boolean, String::from("true")),
            Token::create(TokenType::Boolean, String::from("false")),
            Token::create(TokenType::While, String::from("while")),
            Token::create(TokenType::For, String::from("for")),
            Token::create(TokenType::Funct, String::from("funct")),
            Token::create(TokenType::Num, String::from("num")),
            Token::create(TokenType::Str, String::from("str")),
            Token::create(TokenType::Print, String::from("print")),
            Token::create(TokenType::Return, String::from("return")),
            Token::create(TokenType::Identifier, String::from("test")),
        ],
        tokens
    );

    let contents = String::from("_");
    let mut lexer = Lexer::from_iter(contents.chars());
    assert_eq!(lexer.next(), None);
}
