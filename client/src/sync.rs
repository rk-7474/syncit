use std::collections::HashMap;
use crate::api;
use crate::utils;
use crate::project;


pub async fn send(drawer: &str, path: String) -> Result<(), reqwest::Error> {
    println!("Creating drawer {drawer} at {path}");
    let ignores = project::get_ignores();
    let files: Vec<String> = utils::get_files(path.clone(), ignores); 

    let data_map: HashMap<String, Vec<u8>> = utils::load_files(files);
    let response = api::send_bytes(data_map, drawer).await;
    
    println!("Drawer sended.");

    return response;
}

pub async fn get(drawer: &str, path: String) -> Result<(), reqwest::Error> {
    let response = api::get_bytes(drawer).await;

    println!("Loading drawer {drawer} at {path}");
    let json = response?.text();
    let result = &json.await?;
    let data_map: HashMap<String, Vec<u8>> = serde_json::from_str(result).unwrap();
    for (file_path, buffer) in data_map {
        let f_path = format!("{path}{}", file_path.clone().split_off(2));
        println!("Created {f_path}");
        utils::create_path(&f_path, buffer);
    }

    println!("Drawer acquired.");

    Ok(())
}

// pub fn remove(args: &mut Args) {
//     // let files = utils::get_files(path);  
//     // let data_map = utils::load_files(files);
//     // api::send_bytes(data_map);
// }

