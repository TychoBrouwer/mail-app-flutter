use async_std::sync::{Arc, Mutex};

use crate::database::conn::DBConnection;
use crate::inbox_client::inbox_client::InboxClient;
use crate::my_error::MyError;
use crate::types::session::Client;

impl InboxClient {
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
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let client = &locked_clients[session_id];

        let messages = match DBConnection::get_messages_sorted(
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
}
