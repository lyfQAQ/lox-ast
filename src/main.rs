use std::io::Read;

use error::LoxError;

mod error;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
        std::process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let mut file = std::fs::File::open(path).expect("Error: open file");
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Error: read file content");
    match run(content) {
        Ok(_) => {}
        Err(m) => {
            m.report("".to_string());
            std::process::exit(65);
        }
    }
}

fn run_prompt() {
    print!("> ");
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        match run(line) {
            Ok(_) => {}
            Err(m) => m.report("".to_string()),
        }
        print!("> ");
    }
}

fn run(source: String) -> Result<(), LoxError> {
    for token in source.split_whitespace() {
        println!("{token}");
    }
    Ok(())
}
