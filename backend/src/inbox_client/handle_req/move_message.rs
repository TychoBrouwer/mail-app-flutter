use async_std::sync::{Arc, Mutex};

use crate::database::conn::DBConnection;
use crate::inbox_client::inbox_client::InboxClient;
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

impl InboxClient {
    pub async fn move_message(
        sessions: Arc<Mutex<Vec<Session>>>,
        database_conn: Arc<Mutex<rusqlite::Connection>>,
        session_id: usize,
        clients: Arc<Mutex<Vec<Client>>>,
        mailbox_path: &str,
        message_uid: u32,
        mailbox_path_dest: &str,
    ) -> Result<String, MyError> {
        let clients_2 = Arc::clone(&clients);

        let locked_clients = clients.lock().await;
        dbg!("locked clients");

        if session_id + 1 > locked_clients.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        drop(locked_clients);

        let sessions_2 = Arc::clone(&sessions);

        let mut locked_sessions = sessions.lock().await;
        dbg!("locked sessions");

        let session = &mut locked_sessions[session_id];

        match session.select(mailbox_path).await {
            Ok(_) => {}
            Err(e) => {
                drop(locked_sessions);

                match InboxClient::handle_disconnect(sessions, clients, e).await {
                    Ok(_) => {
                        return Box::pin(InboxClient::move_message(
                            sessions_2,
                            database_conn,
                            session_id,
                            clients_2,
                            mailbox_path,
                            message_uid,
                            mailbox_path_dest,
                        ))
                        .await;
                    }
                    Err(e) => return Err(e),
                }
            }
        };

        match session
            .uid_mv(message_uid.to_string(), mailbox_path_dest)
            .await
        {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error moving message");
                return Err(MyError::Imap(e));
            }
        };

        return InboxClient::move_message_db(
            database_conn,
            session_id,
            clients_2,
            mailbox_path,
            message_uid,
            mailbox_path_dest,
        )
        .await;
    }

    async fn move_message_db(
        database_conn: Arc<Mutex<rusqlite::Connection>>,
        session_id: usize,
        clients: Arc<Mutex<Vec<Client>>>,
        mailbox_path: &str,
        message_uid: u32,
        mailbox_path_dest: &str,
    ) -> Result<String, MyError> {
        let locked_clients = clients.lock().await;
        dbg!("locked clients");
        let client = &locked_clients[session_id];

        match DBConnection::move_message(
            database_conn,
            &client.username,
            &client.address,
            mailbox_path,
            message_uid,
            mailbox_path_dest,
        )
        .await
        {
            Ok(_) => return Ok(format!("\"{}\"", mailbox_path_dest)),
            Err(e) => return Err(e),
        };
    }
}
