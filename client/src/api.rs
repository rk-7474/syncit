use reqwest;
use serde_json;
use std::collections::HashMap;

const ENDPOINT: &'static str = "http://localhost:8080";

pub async fn send_bytes(data_map: HashMap<String, Vec<u8>>, drawer: &str) -> Result<(), reqwest::Error> {
    let url: String = format!("{}/create?id={}", ENDPOINT, drawer);

    println!("Sending data...");

    let json_encoded = serde_json::to_string(&data_map).unwrap();
    let client: reqwest::Client = reqwest::Client::new();
    let response = 
        client.post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_encoded)
        .send().await;

    if let Err(e) = response {
        return Err(e);
    }    

    Ok(())
}

pub async fn get_bytes(drawer: &str) -> Result<reqwest::Response, reqwest::Error> {
    let url: String = format!("{}/get?id={}", ENDPOINT, drawer);

    println!("Getting data...");

    let client: reqwest::Client = reqwest::Client::new();
    let response = 
        client.get(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send().await;
    
    return response;
}