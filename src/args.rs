#[derive(Debug, PartialEq)]
pub struct Args {
    input_file: Option<String>,
    dot: bool,
    compile: bool,
    interpret: bool,
    verbose: bool,
    run: bool,
}

impl Args {
    pub fn create(
        input_file: Option<String>,
        dot: bool,
        compile: bool,
        interpret: bool,
        verbose: bool,
        run: bool,
    ) -> Self {
        Args {
            input_file: input_file,
            dot: dot,
            compile: compile,
            interpret: interpret,
            verbose: verbose,
            run: run,
        }
    }

    // Getter and Setter for input file
    pub fn get_input_file(&self) -> &Option<String> {
        return &self.input_file;
    }

    pub fn set_input_file(&mut self, input_file: Option<String>) {
        self.input_file = input_file;
    }

    // Getter and Setter for dot
    pub fn get_dot(&self) -> &bool {
        return &self.dot;
    }

    pub fn set_dot(&mut self, dot: bool) {
        self.dot = dot;
    }

    // Getter and Setter for compile
    pub fn get_compile(&self) -> &bool {
        return &self.compile;
    }

    pub fn set_compile(&mut self, compile: bool) {
        self.compile = compile;
    }

    // Getter and Setter for interpret
    pub fn get_interpret(&self) -> &bool {
        return &self.interpret;
    }

    pub fn set_interpret(&mut self, interpret: bool) {
        self.interpret = interpret;
    }

    // Getter and Setter for verbose
    pub fn get_verbose(&self) -> &bool {
        return &self.verbose;
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    // Getter and Setter for run
    pub fn get_run(&self) -> &bool {
        return &self.run;
    }

    pub fn set_run(&mut self, run: bool) {
        self.run = run;
    }
}
