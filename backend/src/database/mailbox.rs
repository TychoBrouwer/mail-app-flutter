use async_std::sync::{Arc, Mutex};
use async_std::task;
use rusqlite::{params, Connection};

use crate::database;
use crate::my_error::MyError;

pub async fn insert(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_paths: &Vec<String>,
) -> Result<(), MyError> {
    let mut locked_conn = conn.lock().await;

    let tx = match locked_conn.transaction() {
        Ok(tx) => tx,
        Err(e) => {
            let err = MyError::Sqlite(
                e,
                String::from("Error starting transaction for inserting mailboxes"),
            );
            err.log_error();

            return Err(err);
        }
    };

    for mailbox_path in mailbox_paths {
        match tx.execute(
            "INSERT OR IGNORE INTO mailboxes (
                c_username,
                c_address,
                path
            ) VALUES (?1, ?2, ?3)",
            params![username, address, mailbox_path],
        ) {
            Ok(_) => {}
            Err(e) => {
                let err = MyError::Sqlite(e, String::from("Error inserting mailbox into database"));
                err.log_error();

                return Err(err);
            }
        }
    }

    match tx.commit() {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(
                e,
                String::from("Error committing transaction for inserting mailboxes"),
            );
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

pub async fn get(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
) -> Result<Vec<String>, MyError> {
    let locked_conn = conn.lock().await;

    let mut stmt = match locked_conn
        .prepare_cached("SELECT * FROM mailboxes WHERE c_username = ?1 AND c_address = ?2")
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
