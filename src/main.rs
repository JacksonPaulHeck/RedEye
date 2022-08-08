mod token;
mod test;

use token::Token;
use token::TokenType;

fn print() {
    println!("Hello, world!");

}

fn test_token() {
    use crate::Token;
    use crate::TokenType;
    let token = Token::create(TokenType::Eof, String::from(""));
    assert_eq!(token.get_type(), &TokenType::Eof);
    assert_eq!(token.get_data(), &String::from(""));
}

fn main() {
    print();
    test_token();
}
