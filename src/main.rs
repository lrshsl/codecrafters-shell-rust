#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Check if command is known
        let (cmd_name, _cmd_args) = input.split_once(' ').unwrap_or_else(|| (input.trim(), ""));
        match cmd_name {
            "exit" => break,
            _ => println!("{cmd_name}: command not found"),
        }
    }
}
