use tide::{Request, Response};
use std::collections::HashMap;
use server::database::{Database, State};
use server::database;
use server::auth;
use serde_derive::Deserialize;
use serde_json::{Value, Map};
use uuid::Uuid;

#[derive(Default, Deserialize)]
pub struct GetQuery {
    pub id: String,
}


#[async_std::main]
async fn main() -> tide::Result<()> {
    let db_state = Database::init();
    let mut app = tide::with_state(db_state);
    app.at("/create").post(create);
    app.at("/get").get(get);
    app.at("/update").post(update);
    app.at("/login").post(login);
    println!("Listening on http://127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())  
}

async fn create(mut req: Request<State>) -> tide::Result {
    let data_map: HashMap<String, Vec<u8>> = req.body_json().await?;
    let query: GetQuery = req.query()?;

    println!("Creating drawer {}...", query.id);

    let mut drawer: HashMap<String, String> = HashMap::new();
    
    let mut db = database::get_db(&req);

    for (path, buffer) in data_map {
        let file_id = Uuid::new_v4().to_string();
        drawer.insert(path, file_id.clone());
        let _ = db.update_file(file_id, buffer);
    }

    let drawer_json = serde_json::to_string(&drawer).unwrap();

    let _ = db.update_drawer(query.id, drawer_json);

    println!("Drawer created.");

    Ok(tide::Response::new(200))
}

async fn get(req: Request<State>) -> tide::Result {
    let query: GetQuery = req.query()?;

    let mut db = database::get_db(&req);

    println!("Obtaining drawer {}...", query.id);

    let response = db.get_drawer(query.id);

    if let Err(e) = response {
        return Err(tide::Error::from_str(404, e));
    }

    let drawer_data = response.unwrap();

    let json_map: Value = serde_json::from_str(&drawer_data).unwrap();

    let data_map: Map<String, Value> = serde_json::from_value(json_map).unwrap();

    let mut return_map: HashMap<String, Vec<u8>> = HashMap::new();

    for (path, id) in data_map {
        let buffer = db.get_file(serde_json::from_value(id).unwrap());
        return_map.insert(path, buffer);
    }

    let json_return = serde_json::to_string(&return_map).unwrap();

    let response = Response::builder(200)
        .body(json_return)
        .content_type("application/json")
        .build();

    println!("Drawer sended.");
    
    Ok(response)
}


async fn login(req: Request<State>) -> tide::Result {
    let auth_data: Value = req.body_json().await?;
    let mut db = database::get_db(&req);

    println!("Checking for account drawer {}...", auth_data["username"]);

    let does_user_exist = db.user_exists(auth_data["username"].to_string());

    if does_user_exist {
        db.create_user(auth_data);
    } else {
        db.login(auth_data);
    }

    let drawer_data = response.unwrap();

    let json_map: Value = serde_json::from_str(&drawer_data).unwrap();

    let data_map: Map<String, Value> = serde_json::from_value(json_map).unwrap();

    let mut return_map: HashMap<String, Vec<u8>> = HashMap::new();

    for (path, id) in data_map {
        let buffer = db.get_file(serde_json::from_value(id).unwrap());
        return_map.insert(path, buffer);
    }

    let json_return = serde_json::to_string(&return_map).unwrap();

    let response = Response::builder(200)
        .body(json_return)
        .content_type("application/json")
        .build();

    println!("Drawer sended.");
    
    Ok(response)
}

async fn update(mut req: Request<State>) -> tide::Result {
    let data_map: HashMap<String, Vec<u8>> = req.body_json().await?;
    let mut drawer: HashMap<String, Uuid> = HashMap::new();

    let mut db = database::get_db(&req);

    for (path, buffer) in data_map {
        let file_id =  Uuid::new_v4();
        drawer.insert(path, file_id);
        let _ = db.update_file(file_id.to_string(), buffer);
    }

    Ok(tide::Response::new(200))
}

