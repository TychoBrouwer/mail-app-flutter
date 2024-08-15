use async_std::sync::{Arc, Mutex};
use async_std::task;
use base64::{prelude::BASE64_STANDARD, Engine};
use rusqlite::{params, Connection};

use crate::database;
use crate::my_error::MyError;
use crate::types::message::Message;

pub async fn insert(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    message: &Message,
) -> Result<(), MyError> {
    let html = match String::from_utf8(BASE64_STANDARD.decode(message.html.as_str()).unwrap()) {
        Ok(html) => html,
        Err(e) => {
            let err = MyError::FromUtf8(e, String::from("Error decoding HTML for database"));
            err.log_error();

            return Err(err);
        }
    };

    let decode_text = match BASE64_STANDARD.decode(message.text.as_str()) {
        Ok(decode) => decode,
        Err(e) => {
            let err = MyError::Base64(e, String::from("Error decoding text for database"));
            err.log_error();

            return Err(err);
        }
    };

    let text = match String::from_utf8(decode_text) {
        Ok(text) => text,
        Err(e) => {
            let err = MyError::FromUtf8(e, String::from("Error decoding text bytes for database"));
            err.log_error();

            return Err(err);
        }
    };

    let locked_conn = conn.lock().await;

    match locked_conn.execute(
        "INSERT OR IGNORE INTO messages (
message_uid,
c_username,
c_address,
m_path,
sequence_id,
message_id,
subject,
from_,
sender,
to_,
cc,
bcc,
reply_to,
in_reply_to,
delivered_to,
date_,
received,
flags,
html,
text
) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
        params![
        message.message_uid,
        username,
        address,
        mailbox_path,
        message.sequence_id,
        message.message_id,
        message.subject,
        message.from,
        message.sender,
        message.to,
        message.cc,
        message.bcc,
        message.reply_to,
        message.in_reply_to,
        message.delivered_to,
        message.date,
        message.received,
        message.flags,
        html,
        text
        ],
    ) {
        Ok(_) => {},
        Err(e) => {


            let err = MyError::Sqlite(e, String::from("Error inserting message into database"));
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

pub async fn update_flags(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    message_uid: u32,
    flags_str: &str,
) -> Result<(), MyError> {
    let locked_conn = conn.lock().await;

    match locked_conn.execute(
        "UPDATE messages
SET flags = ?1
WHERE message_uid = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
        params![flags_str, message_uid, username, address, mailbox_path],
    ) {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error updating flags in database"));
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
    sequence_id: u32,
    sequence_id_new: u32,
) -> Result<(), MyError> {
    let locked_conn = conn.lock().await;

    match locked_conn.execute(
        "UPDATE messages
SET sequence_id = ?1
WHERE sequence_id = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
        params![
            sequence_id_new,
            sequence_id,
            username,
            address,
            mailbox_path
        ],
    ) {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error updating sequence id in database"));
            err.log_error();

            return Err(err);
        }
    };

    match locked_conn.execute(
        "UPDATE messages
SET sequence_id = ?1
WHERE message_uid = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
        params![sequence_id, message_uid, username, address, mailbox_path],
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
    username: &str,
    address: &str,
    mailbox_path: &str,
    message_uid: u32,
) -> Result<(), MyError> {
    let locked_conn = conn.lock().await;

    match locked_conn.execute(
        "DELETE FROM messages
WHERE message_uid = ?1 AND c_username = ?2 AND c_address = ?3 AND m_path = ?4",
        params![message_uid, username, address, mailbox_path],
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
