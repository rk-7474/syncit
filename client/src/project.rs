use std::path::Path;
use std::fs::{File, read_to_string};
use std::io::prelude::*;

pub fn get_ignores() -> Vec<String> {
    let ignores_string = get_config_key("ignores");

    let mut ignores: Vec<String> = Vec::new();
    for line in ignores_string.lines() {
        ignores.push(line.to_string());
    }

    return ignores;
}

pub fn get_location() -> String {
    let location = get_config_key("location");

    return location.lines().nth(1).unwrap().to_string();
}

pub fn get_debugging() -> String {
    let location = get_config_key("location");

    return location.lines().nth(1).unwrap().to_string();
}

fn get_config_file() -> String {
    if !Path::new(".sync").exists() {
        return create_config_file();
    } 
    return read_to_string(".sync").unwrap();
}

fn create_config_file() -> String {
    let mut file = File::create(".sync").unwrap();
    let string: String = ".git\n.gitignore\ntarget".to_string();
    let _ = file.write(string.as_bytes());
    return string;
}

fn get_config_key(key: &str) -> String {
    let file = get_config_file();
    let parts = file.split("@");
    
    for part in parts {
        if part.contains(key) {
            return part.to_string();
        }
    }
    return "".to_string();
}