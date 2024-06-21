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
        let (cmd_name, cmd_args) = input.split_once(' ').unwrap_or((&input, ""));
        let cmd_name = cmd_name.trim();
        let cmd_args = cmd_args.trim();

        match cmd_name {
            "exit" => break,
            "echo" => println!("{cmd_args}"),
            _ => println!("{cmd_name}: command not found"),
        }
    }
}
