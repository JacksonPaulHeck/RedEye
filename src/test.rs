#[cfg(test)]
mod tests {
    #[test]
    fn test_token_types() {
        use crate::token::TokenType;

        // Construct TokenType
        let token_type_open_paren:      TokenType = TokenType::OpenParen;
        let token_type_close_paren:     TokenType = TokenType::CloseParen;
        let token_type_open_brace:      TokenType = TokenType::OpenBrace;
        let token_type_close_brace:     TokenType = TokenType::CloseBrace;
        let token_type_blocki:          TokenType = TokenType::Block;
        let token_type_comma:           TokenType = TokenType::Comma;
        let token_type_colon:           TokenType = TokenType::Colon;
        let token_type_dot:             TokenType = TokenType::Dot;
        let token_type_semicolon:       TokenType = TokenType::Semicolon;
        let token_type_equal:           TokenType = TokenType::Equal;
        let token_type_not:             TokenType = TokenType::Not;
        let token_type_nil:             TokenType = TokenType::Nil;
        let token_type_or:              TokenType = TokenType::Or;
        let token_type_and:             TokenType = TokenType::And;
        let token_type_bang_equal:      TokenType = TokenType::BangEqual;
        let token_type_equal_equal:     TokenType = TokenType::EqualEqual;
        let token_type_greater:         TokenType = TokenType::Greater;
        let token_type_greater_equal:   TokenType = TokenType::GreaterEqual;
        let token_type_less:            TokenType = TokenType::Less;
        let token_type_less_equal:      TokenType = TokenType::LessEqual;
        let token_type_addition:        TokenType = TokenType::Addition;
        let token_type_subtract:        TokenType = TokenType::Subtract;
        let token_type_multiply:        TokenType = TokenType::Multiply;
        let token_type_divide:          TokenType = TokenType::Divide;
        let token_type_slash:           TokenType = TokenType::Slash;
        let token_type_star:            TokenType = TokenType::Star;
        let token_type_num:             TokenType = TokenType::Num;
        let token_type_str:             TokenType = TokenType::Str;
        let token_type_if:              TokenType = TokenType::If;
        let token_type_else:            TokenType = TokenType::Else;
        let token_type_for:             TokenType = TokenType::For;
        let token_type_class:           TokenType = TokenType::Class;
        let token_type_return:          TokenType = TokenType::Return;
        let token_type_funct:           TokenType = TokenType::Funct;
        let token_type_while:           TokenType = TokenType::While;
        let token_type_print:           TokenType = TokenType::Print;
        let token_type_string:          TokenType = TokenType::String;
        let token_type_bool:            TokenType = TokenType::Bool;
        let token_type_number:          TokenType = TokenType::Number;
        let token_type_identifier:      TokenType = TokenType::Identifier;
        let token_type_eof:             TokenType = TokenType::Eof;
        let token_type_error:           TokenType = TokenType::Error;
        
        assert_eq!(token_type_open_paren        ,TokenType::OpenParen);
        assert_eq!(token_type_close_paren       ,TokenType::CloseParen);
        assert_eq!(token_type_open_brace        ,TokenType::OpenBrace);
        assert_eq!(token_type_close_brace       ,TokenType::CloseBrace);
        assert_eq!(token_type_blocki            ,TokenType::Block);
        assert_eq!(token_type_comma             ,TokenType::Comma);
        assert_eq!(token_type_colon             ,TokenType::Colon);
        assert_eq!(token_type_dot               ,TokenType::Dot);
        assert_eq!(token_type_semicolon         ,TokenType::Semicolon);
        assert_eq!(token_type_equal             ,TokenType::Equal);
        assert_eq!(token_type_not               ,TokenType::Not);
        assert_eq!(token_type_nil               ,TokenType::Nil);
        assert_eq!(token_type_or                ,TokenType::Or);
        assert_eq!(token_type_and               ,TokenType::And);
        assert_eq!(token_type_bang_equal        ,TokenType::BangEqual);
        assert_eq!(token_type_equal_equal       ,TokenType::EqualEqual);
        assert_eq!(token_type_greater           ,TokenType::Greater);
        assert_eq!(token_type_greater_equal     ,TokenType::GreaterEqual);
        assert_eq!(token_type_less              ,TokenType::Less);
        assert_eq!(token_type_less_equal        ,TokenType::LessEqual);
        assert_eq!(token_type_addition          ,TokenType::Addition);
        assert_eq!(token_type_subtract          ,TokenType::Subtract);
        assert_eq!(token_type_multiply          ,TokenType::Multiply);
        assert_eq!(token_type_divide            ,TokenType::Divide);
        assert_eq!(token_type_slash             ,TokenType::Slash);
        assert_eq!(token_type_star              ,TokenType::Star);
        assert_eq!(token_type_num               ,TokenType::Num);
        assert_eq!(token_type_str               ,TokenType::Str);
        assert_eq!(token_type_if                ,TokenType::If);
        assert_eq!(token_type_else              ,TokenType::Else);
        assert_eq!(token_type_for               ,TokenType::For);
        assert_eq!(token_type_class             ,TokenType::Class);
        assert_eq!(token_type_return            ,TokenType::Return);
        assert_eq!(token_type_funct             ,TokenType::Funct);
        assert_eq!(token_type_while             ,TokenType::While);
        assert_eq!(token_type_print             ,TokenType::Print);
        assert_eq!(token_type_string            ,TokenType::String);
        assert_eq!(token_type_bool              ,TokenType::Bool);
        assert_eq!(token_type_number            ,TokenType::Number);
        assert_eq!(token_type_identifier        ,TokenType::Identifier);
        assert_eq!(token_type_eof               ,TokenType::Eof);
        assert_eq!(token_type_error             ,TokenType::Error);
    }

    #[test]
    fn test_token() {
        use crate::token::Token;
        use crate::token::TokenType;

        // Construct Token
        let mut token = Token::create(TokenType::Error, String::from("Error"));

        // Test Getters
        assert_eq!(token.get_type(), &TokenType::Error);
        assert_eq!(token.get_data(), &String::from("Error"));

        // Test Setters
        token.set_type(TokenType::Eof);
        token.set_data(String::from("Eof"));
        
        assert_eq!(token.get_type(), &TokenType::Eof);
        assert_eq!(token.get_data(), &String::from("Eof"));
    }

    #[test]
    fn test_args() {
        use crate::args::Args;

        // Construct Args
        let mut args = Args::create(None, false, false, false, false, false);

        // Test Getters
        assert_eq!(args.get_input_file(),   &None);
        assert_eq!(args.get_dot(),          &false);
        assert_eq!(args.get_compile(),      &false);
        assert_eq!(args.get_interpret(),    &false);
        assert_eq!(args.get_verbose(),      &false);
        assert_eq!(args.get_run(),          &false);

        // Test Setters
        args.set_input_file(Some(String::from("input.txt")));
        args.set_dot(true);
        args.set_compile(true);
        args.set_interpret(true);
        args.set_verbose(true);
        args.set_run(true);

        assert_eq!(args.get_input_file(),   &Some(String::from("input.txt")));
        assert_eq!(args.get_dot(),          &true);
        assert_eq!(args.get_compile(),      &true);
        assert_eq!(args.get_interpret(),    &true);
        assert_eq!(args.get_verbose(),      &true);
        assert_eq!(args.get_run(),          &true);
    }

    #[test]
    fn integration_test() {
        use crate::main;
        main();
    }
}