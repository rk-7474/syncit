use tide::{Request, Response};
use std::collections::HashMap;
mod database;
use serde_json::{Value, Map};
use database::Database;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct State {
    db: Arc<Mutex<Database>>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let db = Database::init();
    let db_state = State { 
       db: Arc::new(Mutex::new(db))
    };
    let mut app = tide::with_state(db_state);
    app.at("/create").post(create);
    app.at("/get").get(get);
    // app.at("/update").post(update);
    app.listen("127.0.0.1:8080").await?;
    println!("Listening on http://127.0.0.1:8080");
    Ok(())
}

async fn create(mut req: Request<State>) -> tide::Result {
    // let data_map: HashMap<String, Vec<u8>> = req.body_json().await?;
    println!("Request obtained");

    println!("{}", req.body_string().await?);

    // let mut drawer: HashMap<String, Uuid> = HashMap::new();

    // let mutconn = &mut conn;

    // for (path, buffer) in data_map {
    //     let file_id =  Uuid::new_v4();
    //     drawer.insert(path, file_id);
    //     database::update_file(mutconn, file_id, buffer);
    // }

    Ok(tide::Response::new(200))
}

async fn get(req: Request<State>) -> tide::Result {
    let query: database::GetQuery = req.query()?;
    let state = req.state();
    let mut db = state.db.lock().unwrap();
    let drawer_data = db.get_drawer(query.id);

    let json_map: Value = serde_json::from_str(&drawer_data).unwrap();

    let data_map: Map<String, Value> = serde_json::from_value(json_map).unwrap();

    let mut return_map: HashMap<String, Vec<u8>> = HashMap::new();

    for (path, id) in data_map {
        let buffer = db.get_file(serde_json::from_value(id).unwrap());
        return_map.insert(path, buffer);
    }

    let json_return = serde_json::to_string(&return_map).unwrap();

    let response = Response::builder(203)
        .body(json_return)
        .content_type("application/json")
        .build();

    Ok(response)
}

// async fn update(mut req: Request<()>) -> tide::Result {
//     let data_map: HashMap<String, Vec<u8>> = req.body_json().await?;

//     Ok(tide::Response::new(200))
// }