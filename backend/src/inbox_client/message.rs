use async_std::sync::{Arc, Mutex};
use std::u32;

use crate::database;
use crate::inbox_client;
use crate::my_error::MyError;
use crate::types::fetch_mode::FetchMode;
use crate::types::sequence_set::{SequenceSet, StartEnd};
use crate::types::session::{Client, Session};

pub async fn mv(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    client: &Client,
    mailbox_path: &str,
    message_uid: u32,
    mailbox_path_dest: &str,
) -> Result<(), MyError> {
    let sessions_2 = Arc::clone(&sessions);

    match mv_imap(
        sessions,
        session_id,
        client,
        mailbox_path,
        message_uid,
        mailbox_path_dest,
    )
    .await
    {
        Ok(_) => 0,
        Err(e) => return Err(e),
    };

    let sequence_set = SequenceSet {
        nr_messages: None,
        start_end: Some(StartEnd {
            start: u32::MAX - 1,
            end: u32::MAX,
        }),
        idx: None,
    };

    let messages = match inbox_client::messages::get_imap_with_seq(
        sessions_2,
        session_id,
        client,
        mailbox_path_dest,
        &sequence_set,
        FetchMode::ALL,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let message = match messages.first() {
        Some(m) => m,
        None => {
            let err = MyError::String(
                String::from("No message found"),
                String::from("Error fetching message"),
            );
            err.log_error();

            return Err(err);
        }
    };

    let message_uid_new = message.message_uid;
    let sequence_id_new = message.sequence_id;

    match mv_database(
        database_conn,
        client,
        mailbox_path,
        mailbox_path_dest,
        message_uid,
        message_uid_new,
        sequence_id_new,
    )
    .await
    {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    return Ok(());
}

async fn mv_imap(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    client: &Client,
    mailbox_path: &str,
    message_uid: u32,
    mailbox_path_dest: &str,
) -> Result<(), MyError> {
    let sessions_2 = Arc::clone(&sessions);

    let mut locked_sessions = sessions.lock().await;

    let session = &mut locked_sessions[session_id];

    match session.select(mailbox_path).await {
        Ok(_) => {}
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::connect::handle_disconnect(sessions, client, e).await {
                Ok(_) => {
                    return Box::pin(mv_imap(
                        sessions_2,
                        session_id,
                        client,
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
            let err = MyError::Imap(e, String::from("Error moving message"));
            err.log_error();

            return Err(err);
        }
    };

    return Ok(());
}

async fn mv_database(
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
