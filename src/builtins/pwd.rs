pub fn cmd_pwd() {
    println!(
        "{}",
        std::env::current_dir()
            .expect("Why tf do you have no working directory?")
            .display()
    );
}
