pub fn get_user_home() -> String {
    let home = &env!("HOME");
    if home.is_empty() {
        println!("HOME not set");
        "/".to_string()
    } else {
        home.to_string()
    }
}

pub fn cmd_cd(input_split: Vec<&str>) {
    let user_home_directory = get_user_home();
    let arg1 = match input_split.get(1) {
        // Path is supplied
        Some(path) => path.replace('~', &user_home_directory).to_string(),

        // Only `cd` -> go to user home
        None => user_home_directory,
    };
    std::env::set_current_dir(&arg1)
        .unwrap_or_else(|_| println!("{arg1}: No such file or directory"));
}
