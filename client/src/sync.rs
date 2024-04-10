use std::collections::HashMap;
use crate::api;
use crate::utils;
use crate::project;
use crate::auth;


pub async fn send(drawer: &str, path: String) -> Result<reqwest::Response, reqwest::Error> {
    println!("Creating drawer {drawer} at {path}");
    let ignores = project::get_ignores();
    let files: Vec<String> = utils::list_files(path.clone(), ignores); 

    let data_map: HashMap<String, Vec<u8>> = utils::load_files(files);
    let response = api::upload(data_map, drawer).await;
    
    if !response.is_err() {
        println!("Drawer sended.");
    }   

    return response;
}

pub async fn get(drawer: &str, path: String) -> Result<(), reqwest::Error> {
    let response = api::download(drawer).await;

    if let Err(e) = response { 
        return Err(e);
    }   

    println!("Drawer acquired.");

    println!("Loading drawer {drawer} at {path}");
    let json = response?.text();
    let result = &json.await?;
    let data_map: HashMap<String, Vec<u8>> = serde_json::from_str(result).unwrap();
    for (file_path, buffer) in data_map {
        let f_path = format!("{path}{}", file_path.clone().split_off(2));
        println!("Created {f_path}");
        utils::create_path(&f_path, buffer);
    }

    println!("Drawer loaded.");

    Ok(())
}

// pub fn remove(args: &mut Args) {
//     // let files = utils::get_files(path);  
//     // let data_map = utils::load_files(files);
//     // api::send_bytes(data_map);
// }

