use async_imap::error::Error as ImapError;
use async_imap::types::Fetch;
use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};

use crate::database;
use crate::inbox_client;
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

pub async fn modify(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    client: &Client,
    mailbox_path: &str,
    message_uid: u32,
    flags: &Vec<String>,
    add: bool,
) -> Result<(), MyError> {
    match modify_imap(
        Arc::clone(&sessions),
        session_id,
        client,
        mailbox_path,
        message_uid,
        &flags,
        add,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    match database::message::update_flags(
        database_conn,
        &client.username,
        &client.address,
        mailbox_path,
        message_uid,
        &flags,
        add,
    )
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };
}

async fn modify_imap(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    client: &Client,
    mailbox_path: &str,
    message_uid: u32,
    flags: &Vec<String>,
    add: bool,
) -> Result<(), MyError> {
    let sessions_2 = Arc::clone(&sessions);

    let mut locked_sessions = sessions.lock().await;

    let session = &mut locked_sessions[session_id];

    match session.select(mailbox_path).await {
        Ok(_) => {}
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::connect::handle_disconnect(sessions, session_id, client, e).await {
                Ok(_) => {
                    return Box::pin(modify_imap(
                        sessions_2,
                        session_id,
                        client,
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

    let query = flags_query(flags, add);

    let fetches: Vec<Result<Fetch, ImapError>> =
        match session.uid_store(message_uid.to_string(), query).await {
            Ok(e) => e.collect().await,
            Err(e) => {
                let err = MyError::Imap(
                    e,
                    String::from("Error retrieving message while updating flags"),
                );
                err.log_error();

                return Err(err);
            }
        };

    drop(locked_sessions);

    let fetch = if let Some(m) = fetches.first() {
        m
    } else {
        let err = MyError::String(
            String::from("Array out of bounds access"),
            String::from("Error retrieving message while updating flags"),
        );
        err.log_error();

        return Err(err);
    };

    match fetch {
        Ok(_) => return Ok(()),
        Err(e) => {
            let err = MyError::String(e.to_string(), String::from("Error updating message flag"));
            err.log_error();

            return Err(err);
        }
    };
}

fn flags_query(flags: &Vec<String>, add: bool) -> String {
    let mut query = if add { "+" } else { "-" }.to_string();

    query.push_str("FLAGS (");

    for (i, flag) in flags.iter().enumerate() {
        query.push_str("\\");
        query.push_str(&flag);

        if i < flags.len() - 1 {
            query.push_str(" ");
        }
    }

    query.push_str(")");

    return query;
}
