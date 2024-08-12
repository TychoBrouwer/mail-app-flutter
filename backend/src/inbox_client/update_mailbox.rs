use async_imap::error::Error as ImapError;
use async_imap::types::Fetch;
use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};
use std::{collections::HashMap, u32, vec};

use crate::database::db_connection;
use crate::inbox_client;
use crate::inbox_client::parse_message::parse_message;
use crate::my_error::MyError;
use crate::types::sequence_set::{SequenceSet, StartEnd};
use crate::types::session::{Client, Session};

pub async fn update_mailbox(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
) -> Result<String, MyError> {
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

    drop(locked_clients);

    let sessions_2 = Arc::clone(&sessions);
    let clients_2 = Arc::clone(&clients);

    let (highest_seq, highest_seq_uid) =
        match get_highest_seq_imap(sessions_2, session_id, clients_2, mailbox_path).await {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

    let database_conn_2 = Arc::clone(&database_conn);
    let clients_2 = Arc::clone(&clients);

    match get_highest_seq_db(
        database_conn_2,
        session_id,
        clients_2,
        mailbox_path,
        highest_seq_uid,
    )
    .await
    {
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

        let clients_2 = Arc::clone(&clients);
        let sessions_2 = Arc::clone(&sessions);
        let database_conn_2 = Arc::clone(&database_conn);

        let (moved_message_seq_to_uids, new_message_uids) = match get_changed_message_uids(
            sessions_2,
            session_id,
            database_conn_2,
            clients_2,
            mailbox_path,
            &sequence_set,
        )
        .await
        {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

        changed_uids.extend(
            &moved_message_seq_to_uids
                .iter()
                .map(|(seq, _)| *seq)
                .collect::<Vec<u32>>(),
        );
        changed_uids.extend(&new_message_uids);

        if changed_uids.is_empty() {
            break;
        }

        if !new_message_uids.is_empty() {
            let sessions_2 = Arc::clone(&sessions);
            let database_conn_2 = Arc::clone(&database_conn);
            let clients_2 = Arc::clone(&clients);

            match get_new_messages(
                sessions_2,
                session_id,
                database_conn_2,
                clients_2,
                mailbox_path,
                &new_message_uids,
            )
            .await
            {
                Ok(e) => e,
                Err(e) => return Err(e),
            };
        }

        if !moved_message_seq_to_uids.is_empty() {
            let database_conn_2 = Arc::clone(&database_conn);
            let clients_2 = Arc::clone(&clients);

            match update_moved_messeages(
                database_conn_2,
                clients_2,
                session_id,
                mailbox_path,
                &moved_message_seq_to_uids,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }
    }

    let changed_flags_uids =
        match update_flags(sessions, session_id, database_conn, clients, mailbox_path).await {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

    changed_uids.extend(&changed_flags_uids);

    let mut changed_uids_string = String::from("[");
    changed_uids_string.push_str(
        &changed_uids
            .iter()
            .map(|uid| uid.to_string())
            .collect::<Vec<String>>()
            .join(","),
    );
    changed_uids_string.push_str("]");

    return Ok(changed_uids_string);
}

async fn get_highest_seq_imap(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
) -> Result<(u32, u32), MyError> {
    let mut locked_sessions = sessions.lock().await;
    dbg!("locked sessions");

    let session = &mut locked_sessions[session_id];

    let sessions_2 = Arc::clone(&sessions);
    let clients_2 = Arc::clone(&clients);

    let mailbox = match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::handle_disconnect(sessions, clients, e).await {
                Ok(_) => {
                    return Box::pin(get_highest_seq_imap(
                        sessions_2,
                        session_id,
                        clients_2,
                        mailbox_path,
                    ))
                    .await;
                }
                Err(e) => return Err(e),
            }
        }
    };

    let highest_seq = mailbox.exists;

    let fetches: Vec<Result<Fetch, ImapError>> =
        match session.fetch(highest_seq.to_string(), "UID").await {
            Ok(e) => e.collect().await,
            Err(e) => {
                eprintln!("Error fetching messages");

                let err = MyError::Imap(e, String::from("Error fetching messages"));
                err.log_error();

                return Err(err);
            }
        };

    let fetch = if let Some(m) = fetches.first() {
        m
    } else {
        let err = MyError::String(
            String::from("Array out of bounds access"),
            String::from("Error fetching messages"),
        );
        err.log_error();

        return Err(err);
    };

    let fetch = match fetch {
        Ok(f) => f,
        Err(e) => {
            let err = MyError::String(e.to_string(), String::from("Error fetching messages"));
            err.log_error();

            return Err(err);
        }
    };

    let highest_seq_uid = match fetch.uid {
        Some(e) => e,
        None => {
            let err = MyError::String(
                String::from("UID not found in fetch"),
                String::from("Error fetching messages"),
            );
            err.log_error();

            return Err(err);
        }
    };

    return Ok((highest_seq, highest_seq_uid));
}

async fn get_highest_seq_db(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
    highest_seq_uid: u32,
) -> Result<u32, MyError> {
    let locked_clients = clients.lock().await;
    dbg!("locked clients");

    let client = &locked_clients[session_id];

    let messages = match db_connection::get_messages_with_uids(
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
        return Ok(message.unwrap().sequence_id.unwrap_or(u32::MAX));
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
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
    sequence_set: &SequenceSet,
) -> Result<(Vec<(u32, u32)>, Vec<u32>), MyError> {
    let sessions_2 = Arc::clone(&sessions);
    let clients_2 = Arc::clone(&clients);

    let mut locked_sessions = sessions.lock().await;
    dbg!("locked sessions");

    let session = &mut locked_sessions[session_id];

    let mailbox = match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::handle_disconnect(sessions, clients, e).await {
                Ok(_) => {
                    return Box::pin(get_changed_message_uids(
                        sessions_2,
                        session_id,
                        database_conn,
                        clients_2,
                        mailbox_path,
                        sequence_set,
                    ))
                    .await;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    };

    let sequence_set_string = match sequence_set.to_string(mailbox.exists, true) {
        Ok(e) => e,
        Err(e) => return Err(e),
    };

    let fetches: Vec<Result<Fetch, ImapError>> =
        match session.fetch(sequence_set_string, "UID").await {
            Ok(e) => e.collect().await,
            Err(e) => {
                eprintln!("Error fetching messages");

                let err = MyError::Imap(e, String::from("Error fetching messages"));
                err.log_error();

                return Err(err);
            }
        };

    drop(locked_sessions);

    let messages_uids_imap: Vec<u32> = fetches
        .iter()
        .filter_map(|fetch| match fetch {
            Ok(f) => f.uid,
            Err(_) => None,
        })
        .collect();

    let seq_to_uids_imap: HashMap<u32, u32> = fetches
        .iter()
        .filter_map(|fetch| match fetch {
            Ok(f) => f.uid.map(|uid| (f.message, uid)),
            Err(_) => None,
        })
        .collect();

    let locked_clients = clients.lock().await;
    dbg!("locked clients");

    let client = &locked_clients[session_id];

    let messages = match db_connection::get_messages_with_uids(
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

    drop(locked_clients);

    let seq_to_uids_db: HashMap<u32, u32> = messages
        .iter()
        .map(|message| (message.sequence_id.unwrap_or(u32::MAX), message.message_uid))
        .collect();

    let changed_message_uids: Vec<u32> = seq_to_uids_imap
        .iter()
        .filter(|(seq, uid)| seq_to_uids_db.get(seq) != Some(uid))
        .map(|(_, uid)| *uid)
        .collect();

    let new_messages_uids: Vec<u32> = changed_message_uids
        .iter()
        .filter(|uid| seq_to_uids_db.values().find(|v| **v == **uid).is_none())
        .map(|uid| *uid)
        .collect();

    let moved_message_seq_to_uids: Vec<(u32, u32)> = seq_to_uids_imap
        .iter()
        .filter(|(seq, uid)| seq_to_uids_db.get(seq) == Some(uid))
        .map(|(seq, uid)| (*seq, *uid))
        .collect();

    return Ok((moved_message_seq_to_uids, new_messages_uids));
}

async fn get_new_messages(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
    new_message_uids: &Vec<u32>,
) -> Result<(), MyError> {
    let sessions_2 = Arc::clone(&sessions);
    let clients_2 = Arc::clone(&clients);

    let mut locked_sessions = sessions.lock().await;
    dbg!("locked sessions");

    let session = &mut locked_sessions[session_id];

    match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::handle_disconnect(sessions, clients, e).await {
                Ok(_) => {
                    return Box::pin(get_new_messages(
                        sessions_2,
                        session_id,
                        database_conn,
                        clients_2,
                        mailbox_path,
                        new_message_uids,
                    ))
                    .await;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    };

    let uid_set = new_message_uids
        .iter()
        .map(|uid| uid.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let fetches: Vec<Result<Fetch, ImapError>> = match session
        .uid_fetch(&uid_set, "(UID ENVELOPE FLAGS BODY.PEEK[])")
        .await
    {
        Ok(e) => e.collect().await,
        Err(e) => {
            eprintln!("Error fetching messages");

            let err = MyError::Imap(e, String::from("Error fetching messages"));
            err.log_error();

            return Err(err);
        }
    };

    drop(locked_sessions);

    for fetch in fetches {
        let fetch = match fetch {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error updating message flag: {:?}", e);
                let err = MyError::Imap(e, String::from("Error updating message flag"));
                err.log_error();

                return Err(err);
            }
        };

        let message = match parse_message(&fetch) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error parsing message: {:?}", e);
                return Err(e);
            }
        };

        let database_conn_2 = Arc::clone(&database_conn);

        let locked_clients = clients.lock().await;
        dbg!("locked clients");

        let client = &locked_clients[session_id];

        match db_connection::insert_message(
            database_conn_2,
            &client.username,
            &client.address,
            mailbox_path,
            &message,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error inserting message into db: {:?}", e);
                return Err(e);
            }
        };

        drop(locked_clients);
    }

    return Ok({});
}

async fn update_moved_messeages(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
    session_id: usize,
    mailbox_path: &str,
    moved_message_seq_to_uids: &Vec<(u32, u32)>,
) -> Result<(), MyError> {
    let locked_clients = clients.lock().await;

    let client = &locked_clients[session_id];

    for (sequence_id, message_uid) in moved_message_seq_to_uids {
        let database_conn = Arc::clone(&database_conn);

        match db_connection::update_message_sequence_id(
            database_conn,
            &client.username,
            &client.address,
            mailbox_path,
            *message_uid,
            *sequence_id,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => eprintln!("Error moving message: {:?}", e),
        }
    }

    return Ok({});
}

async fn update_flags(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
    mailbox_path: &str,
) -> Result<Vec<u32>, MyError> {
    let sessions_2 = Arc::clone(&sessions);
    let clients_2 = Arc::clone(&clients);

    let mut locked_sessions = sessions.lock().await;
    dbg!("locked sessions");

    let session = &mut locked_sessions[session_id];

    match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => {
            drop(locked_sessions);

            match inbox_client::handle_disconnect(sessions, clients, e).await {
                Ok(_) => {
                    return Box::pin(update_flags(
                        sessions_2,
                        session_id,
                        database_conn,
                        clients_2,
                        mailbox_path,
                    ))
                    .await;
                }
                Err(e) => return Err(e),
            }
        }
    };

    let fetches: Vec<Result<Fetch, ImapError>> = match session.fetch("1:*", "FLAGS").await {
        Ok(e) => e.collect().await,
        Err(e) => {
            let err = MyError::Imap(e, String::from("Error fetching messages"));
            err.log_error();

            return Err(err);
        }
    };

    drop(locked_sessions);

    let mut updated_uids: Vec<u32> = Vec::new();

    for fetch in fetches {
        let fetch = match fetch {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error updating message flag: {:?}", e);
                let err = MyError::Imap(e, String::from("Error updating message flag"));
                err.log_error();

                return Err(err);
            }
        };

        let message_uid = match fetch.uid {
            Some(e) => e,
            None => continue,
        };

        updated_uids.push(message_uid);

        let flags: Vec<_> = fetch.flags().collect();

        let flags_str = inbox_client::parse_message::flags_to_string(&flags);

        let database_conn_2 = Arc::clone(&database_conn);

        let locked_clients = clients.lock().await;
        dbg!("locked clients");

        let client = &locked_clients[session_id];

        match db_connection::update_message_flags(
            database_conn_2,
            &client.username,
            &client.address,
            mailbox_path,
            message_uid,
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
