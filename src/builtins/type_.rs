use crate::constants::BUILTINS;
use crate::utils::search_in_path;

pub fn cmd_type(input_split: Vec<&str>) {
    let arg1 = *input_split.get(1).expect("Type called without arguments");
    if BUILTINS.iter().any(|&s| s == arg1) {
        println!("{arg1} is a shell builtin");
    } else {
        if let Some(path) = search_in_path(arg1) {
            println!("{} is {}", arg1, path.display());
        } else {
            println!("{arg1}: not found");
        }
    }
}
