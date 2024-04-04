use std::fs::File;
use std::io::Read;

pub fn file_to_bytes(filename: &String) -> Vec<u8> {
    let mut file = File::open(filename).unwrap();
    let mut byte_array = Vec::new();
    let _ = file.read_to_end(&mut byte_array);

    return byte_array;
}