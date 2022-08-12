#[cfg(test)]
mod tests {

    
    #[test]
    fn test_token_types() {
        use crate::token::TokenType;

        // Construct TokenType
        let token_type_open_paren: TokenType = TokenType::OpenParen;
        let token_type_close_paren: TokenType = TokenType::CloseParen;
        let token_type_open_brace: TokenType = TokenType::OpenBrace;
        let token_type_close_brace: TokenType = TokenType::CloseBrace;
        let token_type_blocki: TokenType = TokenType::Block;
        let token_type_comma: TokenType = TokenType::Comma;
        let token_type_colon: TokenType = TokenType::Colon;
        let token_type_dot: TokenType = TokenType::Dot;
        let token_type_semicolon: TokenType = TokenType::Semicolon;
        let token_type_equal: TokenType = TokenType::Equal;
        let token_type_not: TokenType = TokenType::Not;
        let token_type_nil: TokenType = TokenType::Nil;
        let token_type_or: TokenType = TokenType::Or;
        let token_type_and: TokenType = TokenType::And;
        let token_type_bang_equal: TokenType = TokenType::BangEqual;
        let token_type_equal_equal: TokenType = TokenType::EqualEqual;
        let token_type_greater: TokenType = TokenType::Greater;
        let token_type_greater_equal: TokenType = TokenType::GreaterEqual;
        let token_type_less: TokenType = TokenType::Less;
        let token_type_less_equal: TokenType = TokenType::LessEqual;
        let token_type_addition: TokenType = TokenType::Addition;
        let token_type_subtract: TokenType = TokenType::Subtract;
        let token_type_multiply: TokenType = TokenType::Multiply;
        let token_type_divide: TokenType = TokenType::Divide;
        let token_type_slash: TokenType = TokenType::Slash;
        let token_type_star: TokenType = TokenType::Star;
        let token_type_num: TokenType = TokenType::Num;
        let token_type_str: TokenType = TokenType::Str;
        let token_type_if: TokenType = TokenType::If;
        let token_type_else: TokenType = TokenType::Else;
        let token_type_for: TokenType = TokenType::For;
        let token_type_class: TokenType = TokenType::Class;
        let token_type_return: TokenType = TokenType::Return;
        let token_type_funct: TokenType = TokenType::Funct;
        let token_type_while: TokenType = TokenType::While;
        let token_type_print: TokenType = TokenType::Print;
        let token_type_string: TokenType = TokenType::String;
        let token_type_bool: TokenType = TokenType::Bool;
        let token_type_number: TokenType = TokenType::Number;
        let token_type_identifier: TokenType = TokenType::Identifier;
        let token_type_eof: TokenType = TokenType::Eof;
        let token_type_error: TokenType = TokenType::Error;

        assert_eq!(token_type_open_paren, TokenType::OpenParen);
        assert_eq!(token_type_close_paren, TokenType::CloseParen);
        assert_eq!(token_type_open_brace, TokenType::OpenBrace);
        assert_eq!(token_type_close_brace, TokenType::CloseBrace);
        assert_eq!(token_type_blocki, TokenType::Block);
        assert_eq!(token_type_comma, TokenType::Comma);
        assert_eq!(token_type_colon, TokenType::Colon);
        assert_eq!(token_type_dot, TokenType::Dot);
        assert_eq!(token_type_semicolon, TokenType::Semicolon);
        assert_eq!(token_type_equal, TokenType::Equal);
        assert_eq!(token_type_not, TokenType::Not);
        assert_eq!(token_type_nil, TokenType::Nil);
        assert_eq!(token_type_or, TokenType::Or);
        assert_eq!(token_type_and, TokenType::And);
        assert_eq!(token_type_bang_equal, TokenType::BangEqual);
        assert_eq!(token_type_equal_equal, TokenType::EqualEqual);
        assert_eq!(token_type_greater, TokenType::Greater);
        assert_eq!(token_type_greater_equal, TokenType::GreaterEqual);
        assert_eq!(token_type_less, TokenType::Less);
        assert_eq!(token_type_less_equal, TokenType::LessEqual);
        assert_eq!(token_type_addition, TokenType::Addition);
        assert_eq!(token_type_subtract, TokenType::Subtract);
        assert_eq!(token_type_multiply, TokenType::Multiply);
        assert_eq!(token_type_divide, TokenType::Divide);
        assert_eq!(token_type_slash, TokenType::Slash);
        assert_eq!(token_type_star, TokenType::Star);
        assert_eq!(token_type_num, TokenType::Num);
        assert_eq!(token_type_str, TokenType::Str);
        assert_eq!(token_type_if, TokenType::If);
        assert_eq!(token_type_else, TokenType::Else);
        assert_eq!(token_type_for, TokenType::For);
        assert_eq!(token_type_class, TokenType::Class);
        assert_eq!(token_type_return, TokenType::Return);
        assert_eq!(token_type_funct, TokenType::Funct);
        assert_eq!(token_type_while, TokenType::While);
        assert_eq!(token_type_print, TokenType::Print);
        assert_eq!(token_type_string, TokenType::String);
        assert_eq!(token_type_bool, TokenType::Bool);
        assert_eq!(token_type_number, TokenType::Number);
        assert_eq!(token_type_identifier, TokenType::Identifier);
        assert_eq!(token_type_eof, TokenType::Eof);
        assert_eq!(token_type_error, TokenType::Error);
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
        assert_eq!(args.get_input_file(), &None);
        assert_eq!(args.get_dot(), &false);
        assert_eq!(args.get_compile(), &false);
        assert_eq!(args.get_interpret(), &false);
        assert_eq!(args.get_verbose(), &false);
        assert_eq!(args.get_run(), &false);

        // Test Setters
        args.set_input_file(Some(String::from("input.txt")));
        args.set_dot(true);
        args.set_compile(true);
        args.set_interpret(true);
        args.set_verbose(true);
        args.set_run(true);

        assert_eq!(args.get_input_file(), &Some(String::from("input.txt")));
        assert_eq!(args.get_dot(), &true);
        assert_eq!(args.get_compile(), &true);
        assert_eq!(args.get_interpret(), &true);
        assert_eq!(args.get_verbose(), &true);
        assert_eq!(args.get_run(), &true);
    }

    
    #[test]
    fn test_run_interpret() {
        use crate::args::Args;
        use crate::ast::ASTNode;
        use crate::ast::ASTNodeType;
        use crate::parse::Parser;
        use crate::run_interpret;
        use crate::token::Token;
        use crate::token::TokenType;
        use crate::ERROR;
        use crate::SUCCESS;

        // Construct Parser
        let mut parser: Parser = Parser::new();
        let mut ast_node = ASTNode::create(Vec::new(), None, ASTNodeType::Empty);
        let args = Args::create(None, false, false, false, false, false);
        parser.set_ast_nodes(vec![ast_node]);
        assert_eq!(run_interpret(&args, parser), ERROR);

        parser = Parser::new();
        ast_node = ASTNode::create(Vec::new(), None, ASTNodeType::Function);
        parser.set_ast_nodes(vec![ast_node]);
        assert_eq!(run_interpret(&args, parser), SUCCESS);

        parser = Parser::new();
        ast_node = ASTNode::create(
            Vec::new(),
            Some(Token::create(TokenType::Error, String::from("error"))),
            ASTNodeType::Empty,
        );
        parser.set_ast_nodes(vec![ast_node]);
        assert_eq!(run_interpret(&args, parser), ERROR);

        parser = Parser::new();
        assert_eq!(run_interpret(&args, parser), SUCCESS);
    }

    
    #[test]
    fn test_run_file() {
        use crate::args::Args;
        use crate::run_file;
        let args = Args::create(None, false, false, false, false, false);
        run_file(args);

        // Test nonexistent file
        let args = Args::create(
            Some(String::from("this file does not exist")),
            false,
            false,
            false,
            false,
            false,
        );
        run_file(args);

        // Test invalid file
        let args = Args::create(
            Some(String::from("examples/test/test_invalid.night")),
            false,
            false,
            false,
            false,
            false,
        );
        run_file(args);

        // Test valid file
        let args = Args::create(
            Some(String::from("examples/test/test.night")),
            false,
            false,
            false,
            false,
            false,
        );
        run_file(args);

        // Test valid lex, invalid parse
        let args = Args::create(
            Some(String::from("examples/test/test_lex_no_parse.night")),
            false,
            false,
            false,
            false,
            false,
        );
        run_file(args);

        // Test interpret
        let args = Args::create(
            Some(String::from("examples/test/test.night")),
            false,
            false,
            true,
            false,
            false,
        );
        run_file(args);

        // Test interpret (error)
        let args = Args::create(
            Some(String::from("examples/test/test_error.night")),
            false,
            false,
            true,
            false,
            false,
        );
        run_file(args);
    }

    
    #[test]
    fn test_get_args() {
        use crate::args::Args;
        use crate::get_args;
        let arguments: Vec<String> = vec![
            String::from("test_program_name"),
            String::from("-i"),
            String::from("-c"),
            String::from("-r"),
            String::from("-v"),
            String::from("-d"),
            String::from("examples/test/test.night"),
        ];
        assert_eq!(
            get_args(arguments),
            Args::create(
                Some(String::from("examples/test/test.night")),
                true,
                true,
                true,
                true,
                true
            )
        );
    }

    
    #[test]
    fn test_run_line() {
        use crate::args::Args;
        use crate::run_line;
        use crate::CONTINUE;
        use crate::ERROR;
        use crate::SUCCESS;
        use std::io::BufRead;
        use std::io::Write;

        let input_file = std::fs::File::open("examples/test/test_input_quit.night")
            .expect("Unable to open file");
        let output_file =
            std::fs::File::create("examples/test/output.night").expect("unable to open file");
        let mut line = String::new();
        let flush_result: Result<(), std::io::Error> = std::io::BufWriter::new(output_file).flush();
        let read_result: Result<usize, std::io::Error> =
            std::io::BufReader::new(input_file).read_line(&mut line);
        let args = Args::create(None, false, false, false, false, false);
        assert_eq!(run_line(&args, line, flush_result, read_result), SUCCESS);

        let input_file =
            std::fs::File::open("examples/test/test_continue.night").expect("Unable to open file");
        let output_file =
            std::fs::File::create("examples/test/output.night").expect("unable to open file");
        line = String::new();
        let flush_result: Result<(), std::io::Error> = std::io::BufWriter::new(output_file).flush();
        let read_result: Result<usize, std::io::Error> =
            std::io::BufReader::new(input_file).read_line(&mut line);
        let args = Args::create(None, false, false, false, false, false);
        assert_eq!(run_line(&args, line, flush_result, read_result), CONTINUE);

        let input_file =
            std::fs::File::open("examples/test/test_empty.night").expect("Unable to open file");
        let output_file =
            std::fs::File::create("examples/test/output.night").expect("unable to open file");
        line = String::new();
        let flush_result: Result<(), std::io::Error> = std::io::BufWriter::new(output_file).flush();
        let read_result: Result<usize, std::io::Error> =
            std::io::BufReader::new(input_file).read_line(&mut line);
        let args = Args::create(None, false, false, false, false, false);
        assert_eq!(run_line(&args, line, flush_result, read_result), ERROR);

        let input_file = std::fs::File::open("").unwrap_err();
        let output_file =
            std::fs::File::create("examples/test/output.night").expect("unable to open file");
        line = String::new();
        let flush_result: Result<(), std::io::Error> = std::io::BufWriter::new(output_file).flush();
        let read_result: Result<usize, std::io::Error> = Err(input_file);
        let args = Args::create(None, false, false, false, false, false);
        assert_eq!(run_line(&args, line, flush_result, read_result), ERROR);

        let input_file =
            std::fs::File::open("examples/test/test.night").expect("Unable to open file");
        let output_file = std::fs::File::create("").unwrap_err();
        line = String::new();
        let flush_result: Result<(), std::io::Error> = Err(output_file);
        let read_result: Result<usize, std::io::Error> =
            std::io::BufReader::new(input_file).read_line(&mut line);
        let args = Args::create(None, false, false, false, false, false);
        assert_eq!(run_line(&args, line, flush_result, read_result), ERROR);
    }

    
    #[test]
    fn test_get_line() {
        use crate::get_line;
        use std::io::BufRead;
        use std::io::Write;

        let input_file =
            std::fs::File::open("examples/test/test.night").expect("Unable to open file");
        let output_file =
            std::fs::File::create("examples/test/output.night").expect("unable to open file");

        let mut in_buffer = std::io::BufReader::new(input_file);
        let mut out_buffer = std::io::BufWriter::new(output_file);

        let mut line = String::new();
        let flush_result: Result<(), std::io::Error> = out_buffer.flush();
        let read_result: Result<usize, std::io::Error> = in_buffer.read_line(&mut line);
        let (line_assert, _, _) = get_line(
            line,
            flush_result,
            read_result,
            &mut out_buffer,
            &mut in_buffer,
        );
        assert_eq!(
            line_assert,
            String::from("funct entry() {\n    \n    return(0);\n}")
        );

        let input_file =
            std::fs::File::open("examples/test/test_continue.night").expect("Unable to open file");
        let output_file =
            std::fs::File::create("examples/test/output.night").expect("unable to open file");

        let mut in_buffer = std::io::BufReader::new(input_file);
        let mut out_buffer = std::io::BufWriter::new(output_file);

        let mut line = String::new();
        let flush_result: Result<(), std::io::Error> = out_buffer.flush();
        let read_result: Result<usize, std::io::Error> = in_buffer.read_line(&mut line);
        let (line_assert, _, _) = get_line(
            line,
            flush_result,
            read_result,
            &mut out_buffer,
            &mut in_buffer,
        );
        assert_eq!(line_assert, String::from("asdfasdf\n"));

        let input_file =
            std::fs::File::open("examples/test/test_block.night").expect("Unable to open file");
        let output_file =
            std::fs::File::create("examples/test/output.night").expect("unable to open file");

        let mut in_buffer = std::io::BufReader::new(input_file);
        let mut out_buffer = std::io::BufWriter::new(output_file);
        let mut line = String::new();
        let flush_result: Result<(), std::io::Error> = out_buffer.flush();
        let read_result: Result<usize, std::io::Error> = in_buffer.read_line(&mut line);
        let (line_assert, _, _) = get_line(
            line,
            flush_result,
            read_result,
            &mut out_buffer,
            &mut in_buffer,
        );
        assert_eq!(line_assert, String::from("{\n}"));

        let input_file =
            std::fs::File::open("examples/test/test_empty.night").expect("Unable to open file");
        let output_file =
            std::fs::File::create("examples/test/output.night").expect("unable to open file");

        let mut in_buffer = std::io::BufReader::new(input_file);
        let mut out_buffer = std::io::BufWriter::new(output_file);
        let mut line = String::new();
        let flush_result: Result<(), std::io::Error> = out_buffer.flush();
        let read_result: Result<usize, std::io::Error> = in_buffer.read_line(&mut line);
        let (line_assert, _, _) = get_line(
            line,
            flush_result,
            read_result,
            &mut out_buffer,
            &mut in_buffer,
        );
        assert_eq!(line_assert, String::from(""));
    }

    
    #[test]
    fn test_repl() {
        use crate::args::Args;
        use crate::repl;
        let args = Args::create(None, false, false, false, false, false);

        let input_file =
            std::fs::File::open("examples/test/test_block.night").expect("Unable to open file");
        let output_file =
            std::fs::File::create("examples/test/output.night").expect("unable to open file");

        let mut in_buffer = std::io::BufReader::new(input_file);
        let mut out_buffer = std::io::BufWriter::new(output_file);
        repl(args, &mut out_buffer, &mut in_buffer);
    }

    
    
    #[test]
    fn integration_test() {
        // use crate::main;
        // main();
    }

    
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
                Token::create(TokenType::Bool, String::from("true")),
                Token::create(TokenType::Bool, String::from("false")),
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
        let file =
            std::fs::File::create("examples/test/test_ast.dot").expect("Unable to open file");
        ast_node.print(0, Some(file));

        ast_node.set_children(vec![None]);

        // Testing print no value (children None)
        ast_node.print(0, None);
        // Testing print with value (children None)
        let file =
            std::fs::File::create("examples/test/test_ast.dot").expect("Unable to open file");
        ast_node.print(0, Some(file));

        // Testing Debug for Code Coverage
        println!("{:#?}", ast_node);
    }

    
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
        parser.push_token(Token::create(TokenType::Bool, String::from("true")));
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

    #[test]
    fn test_interpret_interpret() {
        use crate::args::Args;
        use crate::ast::ChildNode;
        use crate::interpret::Interpreter;
        let interpreter: Interpreter = Interpreter::new();
        let args = Args::create(None, false, false, true, false, false);
        let ast: ChildNode = None;
        assert_eq!(interpreter.interpret(&args, &ast), 0);
        let args = Args::create(None, false, false, false, false, false);
        assert_eq!(interpreter.interpret(&args, &ast), 101);
    }
}
