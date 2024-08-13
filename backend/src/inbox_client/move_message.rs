use async_imap::error::Error as ImapError;
use async_imap::types::Fetch;
use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};
use std::u32;

use crate::database;
use crate::inbox_client;
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

pub async fn move_message(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
    message_uid: u32,
    mailbox_path_dest: &str,
) -> Result<String, MyError> {
    let sessions_2 = Arc::clone(&sessions);

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

    let client = &locked_clients[session_id].clone();
    drop(locked_clients);

    let mut locked_sessions = sessions.lock().await;
    dbg!("locked sessions");

    let session = &mut locked_sessions[session_id];

    match session.select(mailbox_path).await {
        Ok(_) => {}
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::handle_disconnect(sessions, client, e).await {
                Ok(_) => {
                    return Box::pin(move_message(
                        sessions_2,
                        database_conn,
                        session_id,
                        clients,
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
            let err = MyError::Imap(e, String::from("Error moving message"));
            err.log_error();

            return Err(err);
        }
    };

    match session.select(mailbox_path_dest).await {
        Ok(_) => {}
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::handle_disconnect(sessions, client, e).await {
                Ok(_) => {
                    return Box::pin(move_message(
                        sessions_2,
                        database_conn,
                        session_id,
                        clients,
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

    let sequence_set_str = format!("{}:*", u32::MAX);

    let fetches: Vec<Result<Fetch, ImapError>> = match session.fetch(sequence_set_str, "UID").await
    {
        Ok(e) => e.collect().await,
        Err(e) => {
            let err = MyError::Imap(e, String::from("Error fetching messages"));
            err.log_error();

            return Err(err);
        }
    };

    drop(locked_sessions);

    let mut message_uid_new = message_uid;
    let mut sequence_id_new = 0;
    for fetch in fetches {
        match fetch {
            Ok(f) => {
                dbg!("new message uid found");

                message_uid_new = match f.uid {
                    Some(u) => u,
                    None => {
                        let err = MyError::String(
                            String::from("No UID found"),
                            String::from("Error fetching message"),
                        );
                        err.log_error();

                        return Err(err);
                    }
                };

                sequence_id_new = f.message;
            }
            Err(e) => {
                let err = MyError::Imap(e, String::from("Error fetching message"));
                err.log_error();

                return Err(err);
            }
        }
    }

    dbg!(&message_uid);
    dbg!(&message_uid_new);

    return move_message_db(
        database_conn,
        client,
        mailbox_path,
        mailbox_path_dest,
        message_uid,
        message_uid_new,
        sequence_id_new,
    )
    .await;
}

async fn move_message_db(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    mailbox_path_dest: &str,
    message_uid: u32,
    message_uid_new: u32,
    sequence_id_new: u32,
) -> Result<String, MyError> {
    match database::message::change_mailbox(
        database_conn,
        &client.username,
        &client.address,
        mailbox_path,
        mailbox_path_dest,
        message_uid,
        message_uid_new,
        sequence_id_new,
    )
    .await
    {
        Ok(_) => return Ok(format!("\"{}\"", mailbox_path_dest)),
        Err(e) => return Err(e),
    };
}
