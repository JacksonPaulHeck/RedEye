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
    assert_eq!(run_interpret(&args, parser), ERROR);

    let args = Args::create(None, false, false, true, false, false);
    parser = Parser::new();
    ast_node = ASTNode::create(Vec::new(), None, ASTNodeType::Function);
    parser.set_ast_nodes(vec![ast_node]);
    assert_eq!(run_interpret(&args, parser), ERROR);

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

    let input_file =
        std::fs::File::open("examples/test/test_input_quit.night").expect("Unable to open file");
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

    let input_file = std::fs::File::open("examples/test/test.night").expect("Unable to open file");
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

    let input_file = std::fs::File::open("examples/test/test.night").expect("Unable to open file");
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
