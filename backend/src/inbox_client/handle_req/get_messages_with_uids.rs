use rusqlite::Connection;

use crate::database::conn;
use crate::my_error::MyError;
use crate::types::client::Client;

pub fn get_messages_with_uids(
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    message_uids: &Vec<u32>,
) -> Result<String, MyError> {
    let messages =
        match conn::get_messages_with_uids(database_conn, client, mailbox_path, message_uids) {
            Ok(m) => m,
            Err(e) => {
                return Err(e);
            }
        };

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
