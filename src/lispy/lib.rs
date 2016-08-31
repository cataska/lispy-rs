use std::io::{self, Write};

/// Convert a string into a list of tokens.
fn tokenize(s: &str) -> Vec<String> {
    let replaced = s.replace("(", " ( ").replace(")", " ) ");
    let v: Vec<String> = replaced.split_whitespace().map(String::from).collect();
    v
}

fn parse(program: &str) {
    print!("{:?}", tokenize(program));
}

fn read(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input
        }
        Err(error) => panic!("error: {}", error),
    }
}

/// A prompt-read-eval-print loop.
fn repl(prompt: &str) {
    loop {
        let program = read(prompt);
        parse(&program);
    }
}

pub fn run() -> i32 {
    repl("lis.py> ");
    0
}
