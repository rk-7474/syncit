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
        let mut result: Row = self.conn.query_first(
            format!("SELECT data FROM drawers WHERE name = '{}'", drawer)
        ).unwrap().unwrap();

        return result.take("data").unwrap();
    }

    pub fn get_file(&mut self, id: String) -> Vec<u8> {
        let mut result: Row = self.conn.query_first(
            format!("SELECT buffer FROM files WHERE id = '{}'", id)
        ).unwrap().unwrap();
        
        return result.take("buffer").unwrap();
    }

    pub fn update_drawer(&mut self, id: String, data: serde_json::Value) -> mysql::Result<()> {
        self.conn.exec_drop(
            "INSERT INTO drawers (id, data) values (?, ?) ON DUPLICATE KEY UPDATE data = ?",
            (id, data.clone(), data)
        )
    }

    pub fn update_file(&mut self, id: String, buffer: Vec<u8>) -> mysql::Result<()> {
        self.conn.exec_drop(
            "INSERT INTO files (id, buffer) values (?, ?) ON DUPLICATE KEY UPDATE data = ?",
            (id, buffer.clone(), buffer)
        )
    }
}

#[derive(Default, Deserialize)]
pub struct GetQuery {
    pub id: String,
}
