use std::sync::Mutex;
use std::fs::{create_dir_all, File, read_to_string}; 
use std::io::{stdin, stdout, Write};
use std::path::Path;

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

const BASE_PATH: &str = "~/syncit";

fn save_token(token: &String) -> std::io::Result<()> {
    create_dir_all(BASE_PATH).unwrap();
    let mut file = File::create(".db").unwrap();
    file.write(token.as_bytes())?;

    Ok(())
}

pub async fn set_token() -> std::io::Result<()> {
    let mut token = String::new();

    print!("Insert auth token: ");
    let _ = stdout().flush();
    stdin().read_line(&mut token)?;  

    save_token(&token)?;

    Ok(())
}

pub fn read_token() -> String {
    let db_file = format!("{BASE_PATH}/.db");

    if !Path::new(&db_file).exists() {
        return String::new();
    } 

    let str = read_to_string(db_file).unwrap();

    println!("Token: \"{str}\"");

    str
}

