use std::io::Read;

mod token;

static mut HAD_ERROR: bool = false;

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
    run(content);
    if unsafe { HAD_ERROR } {
        std::process::exit(65);
    }
}

fn run_prompt() {
    print!("> ");
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        run(line);
        unsafe {
            HAD_ERROR = false;
        }
        print!("> ");
    }
}

fn run(source: String) {
    for token in source.split_whitespace() {
        println!("{token}");
    }
}

fn error(line: u32, message: String) {
    report(line, String::new(), message);
}

fn report(line: u32, position: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, position, message);
    unsafe {
        HAD_ERROR = true;
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }
}
