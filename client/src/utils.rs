use std::fs;
use std::collections::HashMap;
use crate::blob;

pub fn get_files(path: String) -> Vec<String> {
    let mut file_paths: Vec<String> = Vec::new();

    fn recursive(path: &String, file_paths: &mut Vec<String>) {
        let paths = fs::read_dir(path).unwrap();
        for file in paths {
            let entry = file.unwrap();
            let path = &entry.path().display().to_string();
            if entry.metadata().unwrap().is_dir() {
                recursive(path, file_paths);
            }
            file_paths.push(path.clone());
        }
    }

    recursive(&path, &mut file_paths);

    return file_paths;
}

pub fn load_files(files: Vec<String>, ignores: Vec<String>) -> HashMap<String, Vec<u8>> {
    let mut data_map: HashMap<String, Vec<u8>> = HashMap::new();
    
    for path in files {
        let buffer: Vec<u8> = blob::get_bytes(&path);
        data_map.insert(path, buffer);
    }

    return data_map;
}
