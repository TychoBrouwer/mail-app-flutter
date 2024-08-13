use async_std::sync::{Arc, Mutex};
use rusqlite::{params, Connection};

use crate::my_error::MyError;
use crate::types::session::Client;

pub async fn insert(conn: Arc<Mutex<Connection>>, client: &Client) -> Result<(), MyError> {
    let conn_locked = conn.lock().await;

    match conn_locked.execute(
        "INSERT OR IGNORE INTO connections (
              username,
              password,
              address,
              port
          ) VALUES (?1, ?2, ?3, ?4)",
        params![
            client.username,
            client.password,
            client.address,
            client.port
        ],
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error inserting connection into database"));
            err.log_error();

            return Err(err);
        }
    }
}

pub async fn get_all(conn: Arc<Mutex<Connection>>) -> Result<Vec<Client>, MyError> {
    let conn_locked = conn.lock().await;

    let mut stmt = match conn_locked.prepare("SELECT * FROM connections") {
        Ok(stmt) => stmt,
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error preparing statement at connections"));
            err.log_error();

            return Err(err);
        }
    };

    match stmt.query_map(params![], |row| {
        Ok(Client {
            username: row.get(0).unwrap(),
            password: row.get(1).unwrap(),
            address: row.get(2).unwrap(),
            port: row.get(3).unwrap(),
        })
    }) {
        Ok(rows) => {
            let mut connections: Vec<Client> = Vec::new();

            for row in rows {
                connections.push(match row {
                    Ok(session) => session,
                    Err(_) => continue,
                });
            }

            return Ok(connections);
        }
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error getting connections from database"));
            err.log_error();

            return Err(err);
        }
    };
}

pub async fn remove(conn: Arc<Mutex<Connection>>, client: &Client) -> Result<(), MyError> {
    let conn_locked = conn.lock().await;

    match conn_locked.execute(
        "DELETE FROM connections WHERE username = ?1 AND address = ?2",
        params![client.username, client.address],
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error deleting connection from database"));
            err.log_error();

            return Err(err);
        }
    }
}
