use async_std::sync::{Arc, Mutex};
use async_std::task;
use rusqlite::{params, Connection};

use crate::database;
use crate::my_error::MyError;
use crate::types::session::Client;

pub async fn insert(conn: Arc<Mutex<Connection>>, client: &Client) -> Result<(), MyError> {
    let locked_conn = conn.lock().await;

    match locked_conn.execute(
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
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error inserting connection into database"));
            err.log_error();

            return Err(err);
        }
    };

    drop(locked_conn);
    task::spawn(async move {
        match database::backup(conn).await {
            Ok(_) => {}
            Err(e) => e.log_error(),
        }
    });

    return Ok(());
}

pub async fn get_all(conn: Arc<Mutex<Connection>>) -> Result<Vec<Client>, MyError> {
    let locked_conn = conn.lock().await;

    let mut stmt = match locked_conn.prepare_cached("SELECT * FROM connections") {
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
    let locked_conn = conn.lock().await;

    match locked_conn.execute(
        "DELETE FROM connections WHERE username = ?1 AND address = ?2",
        params![client.username, client.address],
    ) {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error deleting connection from database"));
            err.log_error();

            return Err(err);
        }
    }

    drop(locked_conn);
    task::spawn(async move {
        match database::backup(conn).await {
            Ok(_) => {}
            Err(e) => e.log_error(),
        }
    });

    return Ok(());
}
