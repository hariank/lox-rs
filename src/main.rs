use std::io;
use std::io::Write;
use structopt::StructOpt;

mod lexer;
mod treewalk;
mod utils;

#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    source_path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::from_args();

    match args.source_path {
        Some(source_path) => run_file(source_path.to_str().unwrap()),
        _ => run_prompt(),
    }
}

fn run_file(source_path: &str) {
    let result = std::fs::read_to_string(source_path);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Error reading source {}", error);
        }
    };
    run(content);
}

static PROMPT: &str = ">>>  ";

fn run_prompt() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Ok(num_bytes) = io::stdin().read_line(&mut input) {
            if num_bytes == 0 {
                break;
            }
            run(input.trim().to_string());
        } else {
            println!("Error parsing");
            break;
        };
    }
}

fn run(source: String) {
    treewalk::Interpreter::new().run(source);
}
