use mysql::*;
use mysql::prelude::*;
use serde_derive::Deserialize;
extern crate serde_qs as qs;

pub struct Database {
    pub conn: PooledConn
}

impl Database {
    pub fn init() -> Database {
        let url = "mysql://root:@localhost/syncit";
        let pool = Pool::new(url).unwrap();
        let conn = pool.get_conn().unwrap();
        return Database {
            conn: conn
        }
    }

    pub fn get_drawer(&mut self, drawer: String) -> String {
        let query = self.conn.query_first(
            format!("SELECT data FROM drawers WHERE name = '{}'", drawer)
        ).unwrap();

        let mut result: Row;

        if query != None {
            result = query.unwrap();
            return result.take("data").unwrap();
        }

        return "".to_string();
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
}

#[derive(Default, Deserialize)]
pub struct GetQuery {
    pub id: String,
}
