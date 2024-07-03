#[allow(unused_imports)]
use std::io::{self, Write};

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

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
                let arg1 = *input_split.get(1).expect("Type called without arguments");
                if BUILTINS.iter().any(|&s| s == arg1) {
                    println!("{arg1} is a shell builtin");
                } else {
                    let dirs = env!("PATH").split(':').filter(|s| !s.is_empty());
                    let path = dirs
                        .map(|dir| std::path::Path::new(dir).join(arg1))
                        .find(|p| p.is_file());
                    if let Some(path) = path {
                        println!("{} is {}", arg1, path.display());
                    } else {
                        println!("{arg1}: not found");
                    }
                }
            }
            _ => println!("{cmd_name}: command not found"),
        }
    }
}
