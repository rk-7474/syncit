use std::env::Args;
use std::collections::HashMap;
use crate::api;
use crate::utils;


pub async fn add(args: &mut Args) {
    let arg: Option<String> = args.nth(1);
    let path: String = if arg == None {"./".to_string()} else {arg.unwrap().to_string()};
    let files: Vec<String> = utils::get_files(path);  
    let ignores = project::get_ignores();
    let data_map: HashMap<String, Vec<u8>> = utils::load_files(files, ignores);
    api::send_bytes(data_map, "prova".to_string()).await;
}


// pub fn remove(args: &mut Args) {
//     // let files = utils::get_files(path);  
//     // let data_map = utils::load_files(files);
//     // api::send_bytes(data_map);
// }

