use std::io::{self, Write};
use std::process::Command;

mod builtins;
use builtins::{cmd_cd, cmd_echo, cmd_pwd, cmd_type};

use crate::utils::search_in_path;

mod constants;
mod utils;

fn execute_external_command(cmd_path: std::path::PathBuf, input_split: Vec<&str>) {
    let mut cmd = Command::new(cmd_path);
    for arg in input_split.iter().skip(1) {
        cmd.arg(arg);
    }
    let output = cmd.output().expect("External command failed");
    print!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let argv: Vec<&str> = input
            .split(char::is_whitespace)
            .filter(|s| !s.is_empty())
            .collect();
        let argc = argv.len();
        if argc < 1 {
            continue;
        }
        let cmd_name = argv[0];

        match cmd_name {
            "exit" => break,
            "echo" => cmd_echo(argv),
            "type" => cmd_type(argv),
            "pwd" => cmd_pwd(),
            "cd" => cmd_cd(argv),
            cmd_name => {
                if let Some(path) = search_in_path(cmd_name) {
                    execute_external_command(path, argv);
                } else {
                    println!("{cmd_name}: command not found");
                }
            }
        }
    }
}
