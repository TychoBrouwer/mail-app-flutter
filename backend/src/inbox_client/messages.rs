use async_imap::error::Error as ImapError;
use async_imap::types::Fetch;
use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};

use crate::mime_parser::parser;
use crate::my_error::MyError;
use crate::types::database_request::{DatabaseRequest, MessageIdType, MessageReturnData};
use crate::types::fetch_mode;
use crate::types::message::Message;
use crate::types::sequence_set::SequenceSet;
use crate::types::session::{Client, Session};
use crate::{database, inbox_client};

pub async fn get_database_sorted(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    start: u32,
    end: u32,
) -> Result<Vec<Message>, MyError> {
    let database_request = DatabaseRequest {
        username: client.username.clone(),
        address: client.address.clone(),
        mailbox_path: mailbox_path.to_string(),
        return_data: MessageReturnData::All,
        id_type: MessageIdType::MessageUids,
        sorted: true,
        start: Some(start),
        end: Some(end),
        id_rarray: None,
        flag: None,
        not_flag: None,
    };

    dbg!(&database_request);

    return database::messages::get(database_conn, database_request).await;
}

pub async fn get_database_with_uids(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    message_uids: &Vec<u32>,
) -> Result<Vec<Message>, MyError> {
    let database_request = DatabaseRequest {
        username: client.username.clone(),
        address: client.address.clone(),
        mailbox_path: mailbox_path.to_string(),
        return_data: MessageReturnData::All,
        id_type: MessageIdType::MessageUids,
        sorted: true,
        start: None,
        end: None,
        id_rarray: Some(message_uids.clone()),
        flag: None,
        not_flag: None,
    };

    return database::messages::get(database_conn, database_request).await;
}

pub async fn get_database_with_flag(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    flag: &str,
    not_flag: bool,
) -> Result<Vec<Message>, MyError> {
    let database_request = DatabaseRequest {
        username: client.username.clone(),
        address: client.address.clone(),
        mailbox_path: mailbox_path.to_string(),
        return_data: MessageReturnData::AllWithFlags,
        id_type: MessageIdType::MessageUids,
        sorted: true,
        start: None,
        end: None,
        id_rarray: None,
        flag: Some(flag.to_string()),
        not_flag: Some(not_flag),
    };

    let database_conn_2 = Arc::clone(&database_conn);
    let mut messages = match database::messages::get(database_conn_2, database_request).await {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let list: Vec<u32> = messages.iter().map(|message| message.message_uid).collect();

    let flags = match database::messages::get_flags_with_rarray(
        database_conn,
        &client.username,
        &client.address,
        mailbox_path,
        &list,
        MessageIdType::MessageUids,
    )
    .await
    {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let _ = messages.iter_mut().map(|message| {
        let flags = flags
            .iter()
            .filter_map(|flag| {
                if flag.0 == message.message_uid {
                    Some(flag.1.clone())
                } else {
                    None
                }
            })
            .collect();

        message.flags.push(flags);
    });

    return Ok(messages);
}

pub async fn get_imap_with_uids(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    client: &Client,
    mailbox_path: &str,
    message_uids: &Vec<u32>,
    fetch_mode: fetch_mode::FetchMode,
) -> Result<Vec<Message>, MyError> {
    let sessions_2 = Arc::clone(&sessions);

    let mut locked_sessions = sessions.lock().await;

    let session = &mut locked_sessions[session_id];

    match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::connect::handle_disconnect(sessions, session_id, client, e).await {
                Ok(_) => {
                    return Box::pin(get_imap_with_uids(
                        sessions_2,
                        session_id,
                        client,
                        mailbox_path,
                        message_uids,
                        fetch_mode,
                    ))
                    .await;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    };

    let uid_set = message_uids
        .iter()
        .map(|uid| uid.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let fetches: Vec<Result<Fetch, ImapError>> = match session
        .uid_fetch(&uid_set, fetch_mode::string(fetch_mode))
        .await
    {
        Ok(e) => e.collect().await,
        Err(e) => {
            let err = MyError::Imap(e, String::from("Error fetching messages"));
            err.log_error();

            return Err(err);
        }
    };

    drop(locked_sessions);

    let fetches = fetches
        .iter()
        .filter_map(|fetch| match fetch {
            Ok(f) => Some(f),
            Err(_) => None,
        })
        .collect::<Vec<&Fetch>>();

    let messages = fetches
        .iter()
        .filter_map(|fetch| match parser::parse_fetch(fetch) {
            Ok(m) => Some(m),
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    return Ok(messages);
}

pub async fn get_imap_with_seq(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    client: &Client,
    mailbox_path: &str,
    sequence_set: &SequenceSet,
    fetch_mode: fetch_mode::FetchMode,
) -> Result<Vec<Message>, MyError> {
    let sessions_2 = Arc::clone(&sessions);

    let mut locked_sessions = sessions.lock().await;

    let session = &mut locked_sessions[session_id];

    match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::connect::handle_disconnect(sessions, session_id, client, e).await {
                Ok(_) => {
                    return Box::pin(get_imap_with_seq(
                        sessions_2,
                        session_id,
                        client,
                        mailbox_path,
                        sequence_set,
                        fetch_mode,
                    ))
                    .await;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    };

    let sequence_set_str = match sequence_set.to_string(0, false) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let fetches: Vec<Result<Fetch, ImapError>> = match session
        .fetch(&sequence_set_str, fetch_mode::string(fetch_mode))
        .await
    {
        Ok(e) => e.collect().await,
        Err(e) => {
            let err = MyError::Imap(e, String::from("Error fetching messages"));
            err.log_error();

            return Err(err);
        }
    };

    drop(locked_sessions);

    let fetches = fetches
        .iter()
        .filter_map(|fetch| match fetch {
            Ok(f) => Some(f),
            Err(_) => None,
        })
        .collect::<Vec<&Fetch>>();

    let messages = fetches
        .iter()
        .filter_map(|fetch| match parser::parse_fetch(fetch) {
            Ok(m) => Some(m),
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    return Ok(messages);
}
