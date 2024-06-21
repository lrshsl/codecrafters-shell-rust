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

        let input_split: Vec<&str> = input
            .split(char::is_whitespace)
            .filter(|&s| s != "")
            .collect();
        if input_split.len() < 1 {
            break;
        }
        let cmd_name = input_split[0];

        match cmd_name {
            "exit" => break,
            "echo" => println!(
                "{}",
                input
                    .split_once(char::is_whitespace)
                    .expect("Something's fishy")
                    .1
                    .trim()
            ),
            "type" => {
                let first_arg = *input_split.get(1).expect("Type called without arguments");
                match first_arg {
                    "type" | "echo" | "exit" => println!("{} is a shell builtin", first_arg),
                    _ => println!("{}: not found", first_arg),
                }
            }
            _ => println!("{cmd_name}: command not found"),
        }
    }
}
