use async_imap::error::Error as ImapError;
use async_imap::types::{Fetch, Flag};
use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};

use crate::database::conn::DBConnection;
use crate::inbox_client::{inbox_client::InboxClient, parse_message::flags_to_string};
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

impl InboxClient {
    pub async fn modify_flags(
        sessions: Arc<Mutex<Vec<Session>>>,
        database_conn: Arc<Mutex<rusqlite::Connection>>,
        session_id: usize,
        clients: Arc<Mutex<Vec<Client>>>,
        mailbox_path: &str,
        message_uid: u32,
        flags: &str,
        add: bool,
    ) -> Result<String, MyError> {
        let clients_2 = Arc::clone(&clients);
        let sessions_2 = Arc::clone(&sessions);

        let locked_clients = clients.lock().await;
        dbg!("locked clients");

        if session_id + 1 > locked_clients.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        drop(locked_clients);

        let mut locked_sessions = sessions.lock().await;
        dbg!("locked sessions");

        let session = &mut locked_sessions[session_id];

        match session.select(mailbox_path).await {
            Ok(_) => {}
            Err(e) => {
                drop(locked_sessions);

                match InboxClient::handle_disconnect(sessions, clients, e).await {
                    Ok(_) => {
                        return Box::pin(InboxClient::modify_flags(
                            sessions_2,
                            database_conn,
                            session_id,
                            clients_2,
                            mailbox_path,
                            message_uid,
                            flags,
                            add,
                        ))
                        .await;
                    }
                    Err(e) => return Err(e),
                }
            }
        };

        let query = InboxClient::query(flags, add);

        let fetches: Vec<Result<Fetch, ImapError>> =
            match session.uid_store(message_uid.to_string(), query).await {
                Ok(e) => e.collect().await,
                Err(e) => {
                    eprintln!("Error updating message flag");

                    return Err(MyError::Imap(e));
                }
            };

        drop(locked_sessions);

        let fetch = if let Some(m) = fetches.first() {
            m
        } else {
            return Err(MyError::String("Failed to update flags".to_string()));
        };

        let fetch = match fetch {
            Ok(f) => f,
            Err(e) => return Err(MyError::String(format!("{:?}", e))),
        };

        let updated_flags = fetch.flags().collect();

        return InboxClient::modify_flags_db(
            database_conn,
            session_id,
            clients_2,
            mailbox_path,
            message_uid,
            updated_flags,
        )
        .await;
    }

    async fn modify_flags_db<'a>(
        database_conn: Arc<Mutex<rusqlite::Connection>>,
        session_id: usize,
        clients: Arc<Mutex<Vec<Client>>>,
        mailbox_path: &str,
        message_uid: u32,
        flags: Vec<Flag<'a>>,
    ) -> Result<String, MyError> {
        let flags_str = flags_to_string(&flags);

        let locked_clients = clients.lock().await;
        dbg!("locked clients");
        let client = &locked_clients[session_id];

        match DBConnection::update_message_flags(
            database_conn,
            &client.username,
            &client.address,
            mailbox_path,
            message_uid,
            &flags_str,
        )
        .await
        {
            Ok(_) => return Ok(flags_str),
            Err(e) => return Err(e),
        };
    }

    fn query(flags: &str, add: bool) -> String {
        let mut query = if add { "+" } else { "-" }.to_string();

        query.push_str("FLAGS (");

        for (i, flag) in flags.split(",").enumerate() {
            query.push_str("\\");
            query.push_str(&flag);

            if i < flags.split(",").count() - 1 {
                query.push_str(" ");
            }
        }

        query.push_str(")");

        return query;
    }
}
