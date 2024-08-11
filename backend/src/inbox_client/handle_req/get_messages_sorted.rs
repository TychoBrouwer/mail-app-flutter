use rusqlite::Connection;

use crate::database::conn;
use crate::my_error::MyError;
use crate::types::client::Client;

pub fn get_messages_sorted(
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    start: u32,
    end: u32,
) -> Result<String, MyError> {
    let messages = match conn::get_messages_sorted(database_conn, client, mailbox_path, start, end)
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

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
