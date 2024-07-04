pub fn cmd_echo(input_split: Vec<&str>) {
    println!(
        "{}",
        input_split
            .iter()
            .skip(1)
            .map(|s| *s)
            .collect::<Vec<&str>>()
            .join(" ")
    );
}
