use mysql::*;
use mysql::prelude::*;
use tide::Request;
use std::sync::{Arc, Mutex, MutexGuard};
extern crate serde_qs as qs;
use pwhash::bcrypt;

pub struct Database {
    pub conn: PooledConn
}

#[derive(Clone)]
pub struct State {
    db: Arc<Mutex<Database>>,
}

impl Database {
    pub fn init() -> State {
        let url = "mysql://root:@localhost/syncit";
        let pool = Pool::new(url).unwrap();
        let conn = pool.get_conn().unwrap();
        let db = Database { conn };
        let db_state: State = State { 
            db: Arc::new(Mutex::new(db))
        };
        return db_state;
    }

    pub fn get_drawer(&mut self, drawer: String) -> Result<String, String> {
        let query = self.conn.query_first(
            format!("SELECT data FROM drawers WHERE name = '{}'", drawer)
        ).unwrap();

        let mut result: Row;

        if query != None {
            result = query.unwrap();
            return Ok(result.take("data").unwrap());
        }

        return Err("Drawer not found".to_string());
    }

    pub fn get_file(&mut self, id: String) -> Vec<u8> {
        let mut result: Row = self.conn.query_first(
            format!("SELECT buffer FROM files WHERE id = '{}'", id)
        ).unwrap().unwrap();
        
        return result.take("buffer").unwrap();
    }

    pub fn update_drawer(&mut self, id: String, data: String) {
        let response = self.conn.exec_drop(
            "INSERT INTO drawers (name, data) values (?, ?) ON DUPLICATE KEY UPDATE data = ?",
            (id, data.clone(), data)
        );

        if let Err(e) = response {
            eprintln!("Failed to update file: {}", e);
        }
    }

    pub fn update_file(&mut self, id: String, buffer: Vec<u8>) {
        
        let response = self.conn.exec_drop(
            "INSERT INTO files (id, buffer) values (?, ?) ON DUPLICATE KEY UPDATE buffer = ?",
            (id, buffer.clone(), buffer)
        );

        if let Err(e) = response {
            eprintln!("Failed to update file: {}", e);
        }

    }

    pub fn user_exists(&mut self, username: String) -> bool {
        let result: Row = self.conn.query_first(
            format!("SELECT EXISTS(SELECT 1 FROM users WHERE username = '{username}')")
        ).unwrap().unwrap();
        
        return result.get::<bool, usize>(0).unwrap();
    }

    pub fn create_user(&mut self, auth_data: serde_json::Value) -> bool {
        let hashed_password = bcrypt::hash(auth_data["password"].to_string()).unwrap();
        let username = auth_data["username"].to_string();

        let _: Row = self.conn.query_first(
            format!("INSERT INTO users (username, password) values ('{}', '{}')", username, hashed_password)
        ).unwrap().unwrap();

        true
    }

    pub fn login_user(&mut self, auth_data: serde_json::Value) -> bool {
        let username = auth_data["username"].to_string();
        let password = auth_data["password"].to_string();

        let result: Row = self.conn.query_first(
            format!("SELECT password FROM users WHERE username '{}'", username)
        ).unwrap().unwrap();

        bcrypt::verify(password, &result.get::<String, usize>(0).unwrap())
    }   

    pub fn validate_token(&mut self, token: String) -> bool {
        let result: Row = self.conn.query_first(
            format!("SELECT EXISTS(SELECT 1 FROM sessions WHERE token = '{token}')")
        ).unwrap().unwrap();
        
        return result.get::<bool, usize>(0).unwrap();
    }
}


pub fn get_db<'a>(req: &'a Request<State>) -> MutexGuard<'a, Database> {
    let state = req.state();
    let db = state.db.lock();
    
    let db_guard: MutexGuard<'a, Database> = match db {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    return db_guard;
}

