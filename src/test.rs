use crate::Token;
use crate::TokenType;
#[cfg(test)]
mod tests {
    #[test]
    fn test_token() {
        let token = Token {
            token_type: TokenType::Eof,
            data: String::from(""),
        };
        assert_eq!(token.get_type(), TokenType::Eof);
        assert_eq!(token.data(), String::from(""));
    }
}