use reqwest;
use serde_json;
use std::collections::HashMap;

const ENDPOINT: &'static str = "https://localhost:8080";

pub async fn send_bytes(data_map: HashMap<String, Vec<u8>>, drawer: String) {
    let url: String = format!("{}/create?id={}", ENDPOINT, drawer);
    let json_encoded = serde_json::to_string(&data_map).unwrap();
    println!("{}", json_encoded);
    let client: reqwest::Client = reqwest::Client::new();
    let response = 
        client.post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_encoded)
        .send();

    if response.await.unwrap().status().is_success() {
        println!("Array di byte inviato con successo!");
    } else {
        println!("Errore durante l'invio dell'array di byte");
    }

    // Ok(())
}
