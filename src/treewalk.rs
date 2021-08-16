struct Error {
    message: String,
    line: u32,
}

impl Error {
    fn report(&self) {
        println!("[line {}] {}", self.line, self.message);
    }
}

pub struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { had_error: false }
    }

    pub fn run(&self) {
        // println!("{}", source);
    }
}
