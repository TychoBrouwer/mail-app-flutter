use async_std::sync::{Arc, Mutex};
use async_std::task;
use rusqlite::{params, Connection};

use crate::database;
use crate::my_error::MyError;
use crate::types::session::Client;

pub async fn update_flags(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    message_uid: u32,
    flags: &Vec<String>,
    add: bool,
) -> Result<(), MyError> {
    let mut locked_conn = conn.lock().await;

    let tx = match locked_conn.transaction() {
        Ok(tx) => tx,
        Err(e) => {
            let err = MyError::Sqlite(
                e,
                String::from("Error starting transaction for inserting messages"),
            );
            err.log_error();

            return Err(err);
        }
    };

    let query: String;
    if add {
        query = String::from(
            "INSERT OR IGNORE INTO flags
( message_uid, c_username, c_address, m_path, flag) VALUES (?1, ?2, ?3, ?4, ?5)",
        );
    } else {
        query = String::from(
            "DELETE FROM flags
WHERE message_uid = ?1 AND c_username = ?2 AND c_address = ?3 AND m_path = ?4 AND flag = ?5",
        );
    }

    for flag in flags {
        match tx.execute(
            &query,
            params![message_uid, username, address, mailbox_path, flag],
        ) {
            Ok(_) => {}
            Err(e) => {
                let err = MyError::Sqlite(e, String::from("Error inserting flag into database"));
                err.log_error();

                return Err(err);
            }
        }
    }

    match tx.commit() {
        Ok(_) => {}
        Err(e) => {
            return Err(MyError::Sqlite(
                e,
                String::from("Error committing transaction for modifying flags"),
            ))
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

pub async fn change_mailbox(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    mailbox_path_dest: &str,
    message_uid: u32,
    message_uid_new: u32,
    sequence_id_new: u32,
) -> Result<(), MyError> {
    let locked_conn = conn.lock().await;

    match locked_conn.execute(
        "UPDATE messages
SET m_path = ?1, message_uid = ?2, sequence_id = ?3
WHERE message_uid = ?4 AND c_username = ?5 AND c_address = ?6 AND m_path = ?7",
        params![
            mailbox_path_dest,
            message_uid_new,
            sequence_id_new,
            message_uid,
            username,
            address,
            mailbox_path
        ],
    ) {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error moving message in database"));
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

pub async fn update_sequence_id(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    message_uid: u32,
    sequence_id_new: u32,
) -> Result<(), MyError> {
    let locked_conn = conn.lock().await;
    match locked_conn.execute(
        "UPDATE messages
SET sequence_id = ?1
WHERE message_uid = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
        params![
            sequence_id_new,
            message_uid,
            username,
            address,
            mailbox_path
        ],
    ) {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(
                e,
                String::from("Error clearing sequence id column in database"),
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

pub async fn remove(
    conn: Arc<Mutex<Connection>>,
    client: &Client,
    mailbox_path: &str,
    message_uid: u32,
) -> Result<(), MyError> {
    let locked_conn = conn.lock().await;

    match locked_conn.execute(
        "DELETE FROM messages WHERE message_uid = ?1 AND c_username = ?2 AND c_address = ?3 AND m_path = ?4",
        params![message_uid, &client.username, &client.address, mailbox_path],
    ) {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error deleting message from database"));
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
