use crate::token;

pub struct Lexer<T: Iterator<Item = char>> {
    chars: std::iter::Peekable<T>,
}

impl<T: Iterator<Item = char>> Lexer<T> {
    pub fn from_iter(chars: T) -> Self {
        Self {
            chars: chars.peekable(),
        }
    }
}

impl<T: Iterator<Item = char>> Iterator for Lexer<T> {
    type Item = token::Token;

    fn next(&mut self) -> Option<token::Token> {
        while let Some(_) = self.chars.next_if(|x| x.is_whitespace()) {}

        if let Some(x) = self.chars.next() {
            let mut text = String::new();
            let mut string = String::new();
            text.push(x);
            match x {
                '(' => Some(token::Token::create(token::TokenType::OpenParen, text)),
                ')' => Some(token::Token::create(token::TokenType::CloseParen, text)),
                '{' => Some(token::Token::create(token::TokenType::OpenBrace, text)),
                '}' => Some(token::Token::create(token::TokenType::CloseBrace, text)),
                ',' => Some(token::Token::create(token::TokenType::Comma, text)),
                '.' => Some(token::Token::create(token::TokenType::Dot, text)),
                ':' => Some(token::Token::create(token::TokenType::Colon, text)),
                ';' => Some(token::Token::create(token::TokenType::Semicolon, text)),
                '=' => {
                    if let Some(_) = self.chars.next_if(|x| *x == '=') {
                        text.push('=');
                        Some(token::Token::create(token::TokenType::EqualEqual, text))
                    } else {
                        Some(token::Token::create(token::TokenType::Equal, text))
                    }
                }
                '!' => {
                    if let Some(_) = self.chars.next_if(|x| *x == '=') {
                        text.push('=');
                        Some(token::Token::create(token::TokenType::BangEqual, text))
                    } else {
                        Some(token::Token::create(token::TokenType::Not, text))
                    }
                }
                '<' => {
                    if let Some(_) = self.chars.next_if(|x| *x == '=') {
                        text.push('=');
                        Some(token::Token::create(token::TokenType::LessEqual, text))
                    } else {
                        Some(token::Token::create(token::TokenType::Less, text))
                    }
                }
                '>' => {
                    if let Some(_) = self.chars.next_if(|x| *x == '=') {
                        text.push('=');
                        Some(token::Token::create(token::TokenType::GreaterEqual, text))
                    } else {
                        Some(token::Token::create(token::TokenType::Greater, text))
                    }
                }
                '+' => Some(token::Token::create(token::TokenType::Addition, text)),
                '-' => Some(token::Token::create(token::TokenType::Subtract, text)),
                '*' => Some(token::Token::create(token::TokenType::Multiply, text)),
                '/' => Some(token::Token::create(token::TokenType::Divide, text)),
                '"' => {
                    while let Some(x) = self.chars.next_if(|x| *x != '"') {
                        string.push(x);
                    }
                    if let Some(_) = self.chars.next_if(|x| *x == '"') {}

                    Some(token::Token::create(token::TokenType::String, string))
                }
                _ => {
                    if x.is_numeric() {
                        while let Some(x) = self
                            .chars
                            .next_if(|x| x.is_numeric() || (*x == '.' && !text.contains('.')))
                        {
                            text.push(x);
                        }
                        Some(token::Token::create(token::TokenType::Number, text))
                    } else {
                        if !x.is_alphanumeric() {
                            todo!("X: {}", x);
                        }

                        while let Some(x) = self.chars.next_if(|x| x.is_alphanumeric() || *x == '_')
                        {
                            text.push(x);
                        }

                        match text.as_str() {
                            "if" => Some(token::Token::create(token::TokenType::If, text)),
                            "else" => Some(token::Token::create(token::TokenType::Else, text)),
                            "true" | "false" => {
                                Some(token::Token::create(token::TokenType::Bool, text))
                            }
                            "while" => Some(token::Token::create(token::TokenType::While, text)),
                            "for" => Some(token::Token::create(token::TokenType::For, text)),
                            "funct" => Some(token::Token::create(token::TokenType::Funct, text)),
                            "num" => Some(token::Token::create(token::TokenType::Num, text)),
                            "str" => Some(token::Token::create(token::TokenType::Str, text)),
                            "print" => Some(token::Token::create(token::TokenType::Print, text)),
                            "return" => Some(token::Token::create(token::TokenType::Return, text)),

                            _ => Some(token::Token::create(token::TokenType::Identifier, text)),
                        }
                    }
                }
            }
        } else {
            None
        }
    }
}
