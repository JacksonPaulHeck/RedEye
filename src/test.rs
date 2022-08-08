#[cfg(test)]
mod tests {
    #[test]
    fn test_print() {
        crate::print();
    }
    #[test]
    fn test_token() {
        use crate::Token;
        use crate::TokenType;
        let token = Token::create(TokenType::Eof, String::from(""));
        assert_eq!(token.get_type(), &TokenType::Eof);
        assert_eq!(token.get_data(), &String::from(""));
    }
}