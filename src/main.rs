// RedEye Programming Language
mod args;
mod ast;
mod interpret;
mod lex;
mod parse;
mod test;
mod token;

const SUCCESS: i32 = 0;
const CONTINUE: i32 = -1;
const ERROR: i32 = 101;

fn run_line(
    args: &args::Args,
    line: String,
    flush_result: Result<(), std::io::Error>,
    read_result: Result<usize, std::io::Error>,
) -> i32 {
    let mut parser: parse::Parser = parse::Parser::new();
    match flush_result {
        Ok(_) => match read_result {
            Ok(line_length) => {
                if line == ".quit\n" {
                    println!("Exited!");
                    return SUCCESS;
                } else if line_length > 0 {
                    let lexer = lex::Lexer::from_iter(line.chars());
                    for token in lexer {
                        parser.push_token(token);
                    }

                    parser.push_token(token::Token::create(
                        token::TokenType::Eof,
                        String::from(""),
                    ));

                    parser.parse(&args);
                    for ast in parser.get_ast_nodes() {
                        println!("TODO: Interpret {:#?}", ast);
                    }
                    parser.set_tokens(Vec::<token::Token>::new());
                    return CONTINUE;
                } else {
                    eprintln!("Invalid Input");
                    return ERROR;
                }
            }
            Err(e) => {
                eprintln!("{e:?}");
                return ERROR;
            }
        },
        Err(e) => {
            eprintln!("{e:?}");
            return ERROR;
        }
    }
}

fn get_line(
    mut line: String,
    mut flush_result: Result<(), std::io::Error>,
    mut read_result: Result<usize, std::io::Error>,
    output: &mut dyn std::io::Write,
    input: &mut dyn std::io::Read,
) -> (
    String,
    Result<(), std::io::Error>,
    Result<usize, std::io::Error>,
) {
    use std::io::BufRead;
    let mut in_buffer = std::io::BufReader::new(&mut *input);
    match line.trim().chars().last() {
        Some(x) => {
            if x == '{' {
                loop {
                    print!(".");
                    let mut line_inner = String::new();
                    flush_result = output.flush();
                    read_result = in_buffer.read_line(&mut line_inner);
                    line = line + &line_inner;
                    if line_inner.chars().nth(0).unwrap() == '}' {
                        break;
                    }
                }
            }
            return (line, flush_result, read_result);
        }
        None => return (line, flush_result, read_result),
    }
}

fn repl(args: args::Args, output: &mut dyn std::io::Write, input: &mut dyn std::io::Read) -> i32 {
    let mut result;
    use std::io::BufRead;
    let mut in_buffer = std::io::BufReader::new(input);
    loop {
        let mut line = String::new();
        print!("-> ");
        let mut flush_result: Result<(), std::io::Error> = output.flush();
        let mut read_result: Result<usize, std::io::Error> = in_buffer.read_line(&mut line);
        (line, flush_result, read_result) = get_line(
            line.clone(),
            flush_result,
            read_result,
            output,
            &mut in_buffer,
        );
        result = run_line(&args, line, flush_result, read_result);
        if result != -1 {
            break;
        }
    }
    return result;
}

fn run_interpret(args: &args::Args, parser: parse::Parser) -> i32 {
    let interpreter: interpret::Interpreter = interpret::Interpreter::new();
    for ast_node in parser.get_ast_nodes() {
        match ast_node.get_type() {
            ast::ASTNodeType::Function | ast::ASTNodeType::Declaration => {
                return interpreter.interpret(&args, &Some(Box::new(ast_node.clone())));
            }
            _ => match ast_node.get_operation() {
                Some(_) => {
                    eprintln!("Cannot call functions outside of entry");
                    return ERROR;
                }
                None => {
                    eprintln!("TODO: Main - run_interpret() -> match operation to None");
                    return ERROR;
                }
            },
        }
    }
    return SUCCESS;
}

fn run_file(args: args::Args) -> i32 {
    use std::io::Read;
    let mut result;

    match args.get_input_file() {
        Some(input) => match std::fs::File::open(input) {
            Ok(mut input_file) => {
                let mut contents = String::new();
                match input_file.read_to_string(&mut contents) {
                    Ok(_) => {
                        let lexer: lex::Lexer<std::str::Chars> =
                            lex::Lexer::from_iter(contents.chars());
                        let mut parser: parse::Parser = parse::Parser::new();

                        for token in lexer {
                            parser.push_token(token);
                        }

                        parser.push_token(token::Token::create(
                            token::TokenType::Eof,
                            String::from(""),
                        ));

                        result = parser.parse(&args);

                        // Check for Parse Errror
                        if result != SUCCESS {
                            eprintln!("Error in Parser");
                            return result;
                        }

                        if *args.get_interpret() {
                            result = run_interpret(&args, parser);
                        }
                    }
                    Err(e) => {
                        eprintln!("{e:#?}");
                        result = ERROR;
                    }
                }
            }
            Err(e) => {
                eprintln!("{e:#?}");
                result = ERROR;
            }
        },
        None => {
            eprintln!("Unreachable");
            result = ERROR;
        }
    }
    return result;
}

fn get_args(arguments: Vec<String>) -> args::Args {
    let mut args = args::Args::create(None, false, false, false, false, false);
    for argument in 1..arguments.len() {
        match arguments[argument].as_str() {
            "--dot" => args.set_dot(true),
            "-d" => args.set_dot(true),
            "-v" => args.set_verbose(true),
            "--verbose" => args.set_verbose(true),
            "-c" => args.set_compile(true),
            "--compile" => args.set_compile(true),
            "-i" => args.set_interpret(true),
            "--interpret" => args.set_interpret(true),
            "-r" => args.set_run(true),
            "--run" => args.set_run(true),
            _ => args.set_input_file(Some(String::from(&arguments[argument]))),
        }
    }
    return args;
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let args = get_args(arguments);
    match args.get_input_file() {
        Some(_) => std::process::exit(run_file(args)),
        None => std::process::exit(repl(args, &mut std::io::stdout(), &mut std::io::stdin())),
    }
}
