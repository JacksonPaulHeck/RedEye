mod args;
mod ast;
mod lex;
mod test;
mod token;

fn repl(_args: args::Args) -> i32 {
    loop {
        let mut line = String::new();
        print!("-> ");
        use std::io::Write;
        let mut flush_result: Result<(), std::io::Error> = std::io::stdout().flush();
        let mut read_result: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut line);

        if line.trim().chars().last().unwrap() == '{' {
            loop {
                print!(".");
                let mut line_inner = String::new();
                flush_result = std::io::stdout().flush();
                read_result = std::io::stdin().read_line(&mut line_inner);
                line = line + &line_inner;
                if line_inner.chars().nth(0).unwrap() == '}' {
                    break;
                }
            }
        }
        match flush_result {
            Ok(_) => match read_result {
                Ok(line_length) => {
                    if line == ".quit\n" {
                        println!("Exited!");
                        return 0;
                    } else if line_length > 0 {
                        todo!();
                    } else {
                        eprintln!("Invalid Input");
                        return 101;
                    }
                }
                Err(e) => {
                    eprintln!("{e:?}");
                    return 101;
                }
            },
            Err(e) => {
                eprintln!("{e:?}");
                return 101;
            }
        }
    }
}

fn run_file(args: args::Args) -> i32 {
    use std::io::Read;
    match args.get_input_file() {
        Some(input) => match std::fs::File::open(input) {
            Ok(mut input_file) => {
                let mut contents = String::new();
                match input_file.read_to_string(&mut contents) {
                    Ok(_) => {
                        println!("TODO");
                        return 0;
                    }
                    Err(e) => {
                        eprintln!("{e:#?}");
                        return 101;
                    },
                }
            }
            Err(e) => {
                eprintln!("{e:#?}");
                return 101;
            },
        },
        None => {
            eprintln!("Unreachable");
            return 101;
        }
    }
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
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

    match args.get_input_file() {
        Some(_) => std::process::exit(run_file(args)),
        None => std::process::exit(repl(args)),
    }
}
