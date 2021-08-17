#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub line: u32,
}

impl Error {
    pub fn report(&self) {
        println!("[line {}] {}", self.line, self.message);
    }
}
