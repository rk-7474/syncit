use std::fs;
use std::collections::HashMap;
use crate::blob;

pub fn list_files(path: String, ignores: Vec<String>) -> Vec<String> {
    let mut file_paths: Vec<String> = Vec::new();

    fn recursive(path: &String, file_paths: &mut Vec<String>, ignores: &Vec<String>) {
        let paths = fs::read_dir(path).unwrap();
        
        if ignores.iter().any(|i| format!("./{i}") == *path) {return}
        for file in paths {
            let entry = file.unwrap();
            let path = &entry.path().display().to_string();

            println!("Found {path}");

            if ignores.contains(&format!("./{}", path)) {break};

            if entry.metadata().unwrap().is_dir() {
                recursive(path, file_paths, ignores);
            } else {
                file_paths.push(path.clone());
            }
        }
    }

    recursive(&path, &mut file_paths, &ignores);

    return file_paths;
}

pub fn load_files(files: Vec<String>) -> HashMap<String, Vec<u8>> {
    let mut data_map: HashMap<String, Vec<u8>> = HashMap::new();
    
    for path in files {        
        let buffer: Vec<u8> = blob::file_to_bytes(&path);
        data_map.insert(path, buffer);
    }

    return data_map;
}

pub fn create_path(path: &str, buffer: Vec<u8>) {
    let last = path.rfind('/').unwrap() + 1;
    let folders = path.split_at(last);
    fs::create_dir_all(folders.0).unwrap();
    fs::write(path, buffer).unwrap();
}

