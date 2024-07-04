pub fn search_in_path(cmd_name: &str) -> Option<std::path::PathBuf> {
    let dirs = env!("PATH").split(':').filter(|s| !s.is_empty());
    let path = dirs
        .map(|dir| std::path::Path::new(dir).join(cmd_name))
        .find(|p| p.is_file());
    path
}
