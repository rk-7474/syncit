use reqwest;
use std::collections::HashMap;
use crate::auth::read_token;

const ENDPOINT: &'static str = "http://localhost:8080";

pub async fn upload(data_map: HashMap<String, Vec<u8>>, drawer: &str) -> Result<reqwest::Response, reqwest::Error> {
    let url: String = format!("{ENDPOINT}/create?id={drawer}");

    println!("Sending data...");

    let json_encoded = serde_json::to_string(&data_map).unwrap();
    let client: reqwest::Client = reqwest::Client::new();
    let response = 
        client.post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::AUTHORIZATION, format!("Basic {}", read_token()))
        .body(json_encoded)
        .send().await;

    return response;
}

pub async fn download(drawer: &str) -> Result<reqwest::Response, reqwest::Error> {
    let url: String = format!("{ENDPOINT}/get?id={drawer}");

    println!("Getting data...");

    let client: reqwest::Client = reqwest::Client::new();
    let response = 
        client.get(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send().await;
    
    return response;
} 