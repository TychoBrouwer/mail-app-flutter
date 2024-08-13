use async_std::sync::{Arc, Mutex};

use crate::database;
use crate::my_error::MyError;
use crate::types::session::Client;

pub async fn get_messages_sorted(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
    start: u32,
    end: u32,
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

    let messages = match database::messages::get_sorted(
        database_conn,
        &client.username,
        &client.address,
        mailbox_path,
        start,
        end,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    drop(locked_clients);

    let mut result = String::from("[");
    for (i, message) in messages.iter().enumerate() {
        result.push_str(&message.to_string());

        if i < messages.len() - 1 {
            result.push_str(",");
        }
    }
    result.push_str("]");

    return Ok(result);
}
