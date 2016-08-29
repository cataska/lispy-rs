use std::io::{self, Write};

/// A prompt-read-eval-print loop.
fn repl(prompt: &str) {
    loop {
        let mut input = String::new();

        print!("{}", prompt);
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                print!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

pub fn run() -> i32 {
    repl("lis.py> ");
    0
}
