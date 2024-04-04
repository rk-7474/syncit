use std::io::{stdin,stdout,Write};
use reqwest::Response;
use serde_json::json;
use crate::api;

// pub async fn login() -> Result<Response, String> {
//     let mut username = String::new();
//     let mut password = String::new();
//     print!("Username: ");
//     let _ = stdout().flush();
//     stdin().read_line(&mut username).expect("Invalid username");  

//     print!("Password: ");
//     let _ = stdout().flush();
//     stdin().read_line(&mut password).expect("Invalid password");  
    
//     let auth_data = json!({
//         "username": username,
//         "password": password
//     });
    
//     let response = api::login(auth_data).await;
    
//     if response.is_err() {
//         return Err("Invalid username or password".to_string());
//     }   

//     return Ok(response.unwrap());
// }


pub async fn login() -> Result<Response, String> {
    let mut token = String::new();

    print!("Insert auth token: ");
    let _ = stdout().flush();
    stdin().read_line(&mut token).expect("Invalid input");  

    let response = api::login(token).await;
    
    if response.is_err() {
        return Err("Invalid request".to_string());
    }   

    return Ok(response.unwrap());
}

