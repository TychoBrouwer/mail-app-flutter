use async_std::sync::{Arc, Mutex};
use std::{collections::HashMap, u32, vec};

use crate::database;
use crate::inbox_client;
use crate::my_error::MyError;
use crate::types::fetch_mode::FetchMode;
use crate::types::sequence_set::{SequenceSet, StartEnd};
use crate::types::session::{Client, Session};

struct MessageMoveData {
    sequence_id: u32,
    message_uid: u32,
    sequence_id_new: u32,
}

pub async fn update_mailbox(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
) -> Result<String, MyError> {
    let locked_clients = clients.lock().await;

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

    let sessions_2 = Arc::clone(&sessions);

    let (highest_seq, highest_seq_uid) =
        match get_highest_seq_imap(sessions_2, session_id, client, mailbox_path).await {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

    let database_conn_2 = Arc::clone(&database_conn);

    match get_highest_seq_db(database_conn_2, client, mailbox_path, highest_seq_uid).await {
        Ok(highest_seq_local) => {
            if highest_seq_local == highest_seq {
                return Ok("[]".to_string());
            }
        }
        Err(_) => {}
    };

    let mut changed_uids: Vec<u32> = Vec::new();
    let mut end = 0;

    loop {
        let mut start_end = StartEnd {
            start: end + 1,
            end: end + 50,
        };

        if start_end.start >= highest_seq {
            break;
        }
        if start_end.end > highest_seq {
            start_end.end = highest_seq;
        }

        end += 50;

        let sequence_set = SequenceSet {
            nr_messages: None,
            start_end: Some(start_end),
            idx: None,
        };

        let sessions_2 = Arc::clone(&sessions);
        let database_conn_2 = Arc::clone(&database_conn);

        let (moved_messages, new_message_uids) = match get_changed_message_uids(
            sessions_2,
            session_id,
            database_conn_2,
            client,
            mailbox_path,
            &sequence_set,
        )
        .await
        {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

        changed_uids.extend(
            &moved_messages
                .iter()
                .map(|m| m.message_uid)
                .collect::<Vec<u32>>(),
        );
        changed_uids.extend(&new_message_uids);

        if changed_uids.is_empty() {
            break;
        }

        if !new_message_uids.is_empty() {
            let sessions_2 = Arc::clone(&sessions);
            let database_conn_2 = Arc::clone(&database_conn);

            match get_new_messages(
                sessions_2,
                session_id,
                database_conn_2,
                client,
                mailbox_path,
                &new_message_uids,
            )
            .await
            {
                Ok(e) => e,
                Err(e) => return Err(e),
            };
        }

        if !moved_messages.is_empty() {
            let database_conn_2 = Arc::clone(&database_conn);

            match update_moved_messeages(database_conn_2, client, mailbox_path, &moved_messages)
                .await
            {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }
    }

    let changed_flags_uids =
        match update_flags(sessions, session_id, database_conn, client, mailbox_path).await {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

    changed_uids.extend(&changed_flags_uids);

    let changed_uids_string = String::from("[")
        + &changed_uids
            .iter()
            .map(|uid| uid.to_string())
            .collect::<Vec<String>>()
            .join(",")
        + "]";

    return Ok(changed_uids_string);
}

async fn get_highest_seq_imap(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    client: &Client,
    mailbox_path: &str,
) -> Result<(u32, u32), MyError> {
    let sequence_set = SequenceSet {
        nr_messages: None,
        start_end: Some(StartEnd {
            start: u32::MAX - 1,
            end: u32::MAX,
        }),
        idx: None,
    };

    let messages = match inbox_client::messages::get_imap_with_seq(
        sessions,
        session_id,
        client,
        mailbox_path,
        &sequence_set,
        FetchMode::UID,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let highest_seq: u32;
    let highest_seq_uid: u32;

    let message = messages.first();
    if message.is_some() {
        highest_seq = message.unwrap().sequence_id;
        highest_seq_uid = message.unwrap().message_uid;
    } else {
        let err = MyError::String(
            String::from("Message not found in db"),
            String::from("Failed to get message from db"),
        );
        err.log_error();

        return Err(err);
    }

    return Ok((highest_seq, highest_seq_uid));
}

async fn get_highest_seq_db(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    highest_seq_uid: u32,
) -> Result<u32, MyError> {
    let messages = match database::messages::get_with_uids(
        database_conn,
        &client.username,
        &client.address,
        mailbox_path,
        &vec![highest_seq_uid],
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let message = messages.first();
    if message.is_some() {
        return Ok(message.unwrap().sequence_id);
    } else {
        let err = MyError::String(
            String::from("Message not found in db"),
            String::from("Failed to get message from db"),
        );
        err.log_error();

        return Err(err);
    }
}

async fn get_changed_message_uids(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    sequence_set: &SequenceSet,
) -> Result<(Vec<MessageMoveData>, Vec<u32>), MyError> {
    let messages_imap = match inbox_client::messages::get_imap_with_seq(
        sessions,
        session_id,
        client,
        mailbox_path,
        sequence_set,
        FetchMode::UID,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let messages_uids_imap: Vec<u32> = messages_imap.iter().map(|m| m.message_uid).collect();
    let seq_to_uids_imap: HashMap<u32, u32> = messages_imap
        .iter()
        .map(|message| (message.sequence_id, message.message_uid))
        .collect();

    let messages_db = match database::messages::get_with_uids(
        database_conn,
        &client.username,
        &client.address,
        mailbox_path,
        &messages_uids_imap,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let moved_messages: Vec<MessageMoveData> = messages_db
        .iter()
        .filter(|m| seq_to_uids_imap.get(&m.sequence_id) != Some(&m.message_uid))
        .map(|m| MessageMoveData {
            sequence_id: m.sequence_id,
            message_uid: m.message_uid,
            sequence_id_new: *seq_to_uids_imap.get(&m.sequence_id).unwrap(),
        })
        .collect::<Vec<MessageMoveData>>();

    let new_messages_uids: Vec<u32> = messages_uids_imap
        .iter()
        .filter(|uid| {
            messages_db
                .iter()
                .find(|m| m.message_uid == **uid)
                .is_none()
        })
        .map(|uid| *uid)
        .collect();

    return Ok((moved_messages, new_messages_uids));
}

async fn get_new_messages(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    new_message_uids: &Vec<u32>,
) -> Result<(), MyError> {
    let messages = match inbox_client::messages::get_imap_with_uids(
        sessions,
        session_id,
        client,
        mailbox_path,
        new_message_uids,
        FetchMode::ALL,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    match database::messages::insert(
        database_conn,
        &client.username,
        &client.address,
        mailbox_path,
        &messages,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    return Ok(());
}

async fn update_moved_messeages(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    moved_messages: &Vec<MessageMoveData>,
) -> Result<(), MyError> {
    for moved_message in moved_messages {
        let database_conn = Arc::clone(&database_conn);

        match database::message::update_sequence_id(
            database_conn,
            &client.username,
            &client.address,
            mailbox_path,
            moved_message.message_uid,
            moved_message.sequence_id,
            moved_message.sequence_id_new,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    return Ok(());
}

async fn update_flags(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
) -> Result<Vec<u32>, MyError> {
    let messages = match inbox_client::messages::get_imap_with_seq(
        sessions,
        session_id,
        client,
        mailbox_path,
        &SequenceSet {
            nr_messages: None,
            start_end: Some(StartEnd {
                start: 1,
                end: u32::MAX,
            }),
            idx: None,
        },
        FetchMode::FLAGS,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let updated_uids = messages.iter().map(|m| m.message_uid).collect::<Vec<u32>>();

    for message in messages {
        let flags_str = message.flags;

        let database_conn_2 = Arc::clone(&database_conn);

        match database::message::update_flags(
            database_conn_2,
            &client.username,
            &client.address,
            mailbox_path,
            message.message_uid,
            &flags_str,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    return Ok(updated_uids);
}
