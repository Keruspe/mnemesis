use std::io::{self, Write};

pub fn prompt_for_input(msg: &str) -> String {
    print!("{} ", msg);
    io::stdout().flush().expect("Failed flushing stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed reading from stdin");
    input.trim().to_string()
}
