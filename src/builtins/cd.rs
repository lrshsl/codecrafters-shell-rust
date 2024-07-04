pub fn cmd_cd(input_split: Vec<&str>) {
    let arg1 = *input_split.get(1).unwrap_or_else(|| {
        let home = &env!("HOME");
        if home.is_empty() {
            println!("HOME not set");
            &"/"
        } else {
            &home
        }
    });
    std::env::set_current_dir(arg1)
        .unwrap_or_else(|_| println!("{arg1}: No such file or directory"));
}
