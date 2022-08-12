use crate::args;
use crate::ast;
use crate::token;
pub struct Interpreter {
    variables: std::collections::HashMap<String, Option<token::Token>>,
    functions: std::collections::HashMap<String, Vec<ast::ChildNode>>,
}
impl Interpreter {
    pub fn new() -> Self {
        return Interpreter {
            variables: std::collections::HashMap::new(),
            functions: std::collections::HashMap::new(),
        };
    }

    fn visit(&self, ast: &ast::ChildNode) -> i32 {
        return 0;
    }

    pub fn interpret(&self, args: &args::Args, ast: &ast::ChildNode) -> i32 {
        if *args.get_interpret() {
            return self.visit(ast);
        }
        return 101;
    }
}
