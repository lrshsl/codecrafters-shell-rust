#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

fn search_absolute_path_in_path_var(cmd_name: &str) -> Option<std::path::PathBuf> {
    let dirs = env!("PATH").split(':').filter(|s| !s.is_empty());
    let path = dirs
        .map(|dir| std::path::Path::new(dir).join(cmd_name))
        .find(|p| p.is_file());
    path
}

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
                    if let Some(path) = search_absolute_path_in_path_var(arg1) {
                        println!("{} is {}", arg1, path.display());
                    } else {
                        println!("{arg1}: not found");
                    }
                }
            }
            cmd_name => {
                if let Some(path) = search_absolute_path_in_path_var(cmd_name) {
                    let mut args = input_split.iter();
                    args.next();
                    let mut cmd = Command::new(path);
                    for arg in args {
                        cmd.arg(arg);
                    }
                    let output = cmd.output().expect("External command failed");
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    println!("{cmd_name}: command not found");
                }
            }
        }
    }
}
