use crate::lexer;

pub struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { had_error: false }
    }

    pub fn run(&mut self, source: String) {
        match lexer::get_tokens(source) {
            Ok(tokens) => println!("Parsed {} tokens!", tokens.len()),
            Err(_errors) => self.had_error = true,
        }
    }
}
