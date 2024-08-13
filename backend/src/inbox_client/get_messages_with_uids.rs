use async_std::sync::{Arc, Mutex};

use crate::database;
use crate::my_error::MyError;
use crate::types::session::Client;

pub async fn get_messages_with_uids(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
    message_uids: &Vec<u32>,
) -> Result<String, MyError> {
    let locked_clients = clients.lock().await;
    dbg!("locked clients");

    if session_id + 1 > locked_clients.len() {
        let err = MyError::String(
            String::from("Out of bounds array access"),
            String::from("Invalid session ID"),
        );
        err.log_error();

        return Err(err);
    }

    let client = &locked_clients[session_id];

    let messages = match database::messages::get_with_uids(
        database_conn,
        &client.username,
        &client.address,
        mailbox_path,
        message_uids,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => {
            return Err(e);
        }
    };

    drop(locked_clients);

    let mut response = String::from("[");

    for (i, message) in messages.iter().rev().enumerate() {
        response.push_str(&message.to_string());

        if i < messages.len() - 1 {
            response.push_str(",");
        }
    }

    response.push_str("]");

    return Ok(response);
}
