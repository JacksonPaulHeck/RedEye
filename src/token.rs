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

    pub fn get_type(&self) -> &TokenType {
        return &self.token_type;
    }

    pub fn get_data(&self) -> &String {
        return &self.data;
    }
}