#[derive(Debug, PartialEq)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Block,
    Comma,
    Colon,
    Dot,
    Semicolon,
    Equal,
    Not,
    Nil,
    Or,
    And,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Addition,
    Subtract,
    Multiply,
    Divide,
    Slash,
    Star,
    Num,
    Str,
    If,
    Else,
    For,
    Class,
    Return,
    Funct,
    While,
    Print,
    String,
    Bool,
    Number,
    Identifier,
    Eof,
    Error,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    data: String,
}

impl Token {
    pub fn create(token_type: TokenType, data: String) -> Self {
        return Token {
            token_type: token_type,
            data: data,
        };
    }

    // Getter and Setter for token_type
    pub fn get_type(&self) -> &TokenType {
        return &self.token_type;
    }

    pub fn set_type(&mut self, token_type: TokenType) {
        self.token_type = token_type;
    }

    // Getter and Setter for data
    pub fn get_data(&self) -> &String {
        return &self.data;
    }

    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }
}
