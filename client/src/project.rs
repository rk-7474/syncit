pub fn get_ignores() -> Vec<String> {
    let data = get_sync_file().read_lines();
    let ignores: Vec<String> = Vec::new();
    for line in data {
        ignores.push(line.to_string());
    }
    return ignores;
}

fn get_sync_file() -> String {
    if !Path::new("/etc/hosts").exists() {
        return create_sync_file();
    } 
    let mut file = File::open(".sync").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return contents;
}

fn create_sync_file() -> String {
    let mut file = File::create(".sync")?;
    let string = ".git\n.gitignore\ntarget".to_string();
    file.write_all(string)?;
    return string;
}