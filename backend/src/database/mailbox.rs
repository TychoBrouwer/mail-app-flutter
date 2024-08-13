use async_std::sync::{Arc, Mutex};
use rusqlite::{params, Connection};

use crate::my_error::MyError;

pub async fn insert(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
) -> Result<(), MyError> {
    let conn_locked = conn.lock().await;
    
    match conn_locked.execute(
        "INSERT OR IGNORE INTO mailboxes (
              c_username,
              c_address,
              path
          ) VALUES (?1, ?2, ?3)",
        params![username, address, mailbox_path],
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error inserting mailbox into database"));
            err.log_error();

            return Err(err);
        }
    }
}

pub async fn get(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
) -> Result<Vec<String>, MyError> {
    let conn_locked = conn.lock().await;
    
    let mut stmt = match conn_locked
        .prepare("SELECT * FROM mailboxes WHERE c_username = ?1 AND c_address = ?2")
    {
        Ok(stmt) => stmt,
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error preparing statement at mailboxes"));
            err.log_error();

            return Err(err);
        }
    };

    let mut mailboxes: Vec<String> = Vec::new();

    match stmt.query_map(params![username, address], |row| row.get(2)) {
        Ok(rows) => {
            for row in rows {
                mailboxes.push(row.unwrap());
            }
        }
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error getting mailboxes from database"));
            err.log_error();

            return Err(err);
        }
    }

    return Ok(mailboxes);
}
