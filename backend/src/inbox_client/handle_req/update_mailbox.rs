use async_imap::error::Error as ImapError;
use async_imap::types::Fetch;
use async_imap::Session;
use async_native_tls::TlsStream;
use async_std::net::TcpStream;
use futures::StreamExt;
use rusqlite::Connection;
use std::{collections::HashMap, u32, vec};

use crate::database::conn;
use crate::inbox_client::inbox_client::handle_disconnect;
use crate::my_error::MyError;
use crate::types::sequence_set::{SequenceSet, StartEnd};
use crate::{
    inbox_client::parse_message::{flags_to_string, parse_message},
    types::client::Client,
};

pub async fn update_mailbox(
    session: &mut Session<TlsStream<TcpStream>>,
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
) -> Result<String, MyError> {
    let (highest_seq, highest_seq_uid) =
        match get_highest_seq_imap(session, client, mailbox_path).await {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

    match get_highest_seq_db(database_conn, client, mailbox_path, highest_seq_uid) {
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

        let (moved_message_seq_to_uids, new_message_uids) = match get_changed_message_uids(
            session,
            database_conn,
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
            match get_new_messages(
                session,
                database_conn,
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

        if !moved_message_seq_to_uids.is_empty() {
            match update_moved_messeages(
                database_conn,
                client,
                mailbox_path,
                &moved_message_seq_to_uids,
            ) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }
    }

    let changed_flags_uids = match update_flags(session, database_conn, client, mailbox_path).await
    {
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
    session: &mut Session<TlsStream<TcpStream>>,
    client: &Client,
    mailbox_path: &str,
) -> Result<(u32, u32), MyError> {
    let mailbox = match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => match handle_disconnect(client, e).await {
            Ok(_) => {
                return Box::pin(get_highest_seq_imap(session, client, mailbox_path)).await;
            }
            Err(e) => return Err(e),
        },
    };

    let highest_seq = mailbox.exists;

    let fetches: Vec<Result<Fetch, ImapError>> =
        match session.fetch(highest_seq.to_string(), "UID").await {
            Ok(e) => e.collect().await,
            Err(e) => {
                eprintln!("Error fetching messages");

                return Err(MyError::Imap(e));
            }
        };

    let mut first_fetch: Option<Fetch> = None;
    for fetch in fetches {
        first_fetch = match fetch {
            Ok(first_fetch) => Some(first_fetch),
            Err(e) => {
                eprintln!("Failed to get last message in inbox from imap");

                return Err(MyError::Imap(e));
            }
        };

        break;
    }

    let fetch = match first_fetch {
        Some(e) => e,
        None => {
            return Err(MyError::String(
                "Failed to get last message in inbox from imap".to_string(),
            ));
        }
    };

    let highest_seq_uid = match fetch.uid {
        Some(e) => e,
        None => {
            return Err(MyError::String(
                "Failed to get last message in inbox from imap".to_string(),
            ));
        }
    };

    return Ok((highest_seq, highest_seq_uid));
}

fn get_highest_seq_db(
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    highest_seq_uid: u32,
) -> Result<u32, MyError> {
    let messages = match conn::get_messages_with_uids(
        database_conn,
        client,
        mailbox_path,
        &vec![highest_seq_uid],
    ) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let message = messages.first();
    if message.is_some() {
        return Ok(message.unwrap().sequence_id.unwrap_or(u32::MAX));
    } else {
        return Err(MyError::String(
            "Failed to get last message in inbox from db".to_string(),
        ));
    }
}

async fn get_changed_message_uids(
    session: &mut Session<TlsStream<TcpStream>>,
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    sequence_set: &SequenceSet,
) -> Result<(Vec<(u32, u32)>, Vec<u32>), MyError> {
    let mailbox = match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => match handle_disconnect(client, e).await {
            Ok(_) => {
                return Box::pin(get_changed_message_uids(
                    session,
                    database_conn,
                    client,
                    mailbox_path,
                    sequence_set,
                ))
                .await;
            }
            Err(e) => {
                return Err(e);
            }
        },
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

                return Err(MyError::Imap(e));
            }
        };

    let messages_uids_imap: Vec<u32> = fetches
        .iter()
        .filter_map(|fetch| match fetch {
            Ok(fetch) => fetch.uid,
            Err(_) => None,
        })
        .collect();

    let seq_to_uids_imap: HashMap<u32, u32> = fetches
        .iter()
        .filter_map(|fetch| match fetch {
            Ok(fetch) => fetch.uid.map(|uid| (fetch.message, uid)),
            Err(_) => None,
        })
        .collect();

    let messages = match conn::get_messages_with_uids(
        database_conn,
        client,
        mailbox_path,
        &messages_uids_imap,
    ) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

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
    session: &mut Session<TlsStream<TcpStream>>,
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    new_message_uids: &Vec<u32>,
) -> Result<(), MyError> {
    match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => match handle_disconnect(client, e).await {
            Ok(_) => {
                return Box::pin(get_new_messages(
                    session,
                    database_conn,
                    client,
                    mailbox_path,
                    new_message_uids,
                ))
                .await;
            }
            Err(e) => {
                return Err(e);
            }
        },
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

            return Err(MyError::Imap(e));
        }
    };

    for fetch in fetches {
        let fetch = match fetch {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(MyError::Imap(e));
            }
        };

        let message = match parse_message(&fetch) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error parsing message: {:?}", e);
                return Err(e);
            }
        };

        match conn::insert_message(database_conn, client, mailbox_path, &message) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error inserting message into db: {:?}", e);
                return Err(e);
            }
        };
    }

    return Ok({});
}

fn update_moved_messeages(
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    moved_message_seq_to_uids: &Vec<(u32, u32)>,
) -> Result<(), MyError> {
    for (sequence_id, message_uid) in moved_message_seq_to_uids {
        match conn::update_message_sequence_id(
            database_conn,
            client,
            mailbox_path,
            *message_uid,
            *sequence_id,
        ) {
            Ok(_) => {}
            Err(e) => eprintln!("Error moving message: {:?}", e),
        }
    }

    return Ok({});
}

async fn update_flags(
    session: &mut Session<TlsStream<TcpStream>>,
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
) -> Result<Vec<u32>, MyError> {
    match session.select(mailbox_path).await {
        Ok(m) => m,
        Err(e) => match handle_disconnect(client, e).await {
            Ok(_) => {
                return Box::pin(update_flags(session, database_conn, client, mailbox_path)).await;
            }
            Err(e) => return Err(e),
        },
    };

    let fetches: Vec<Result<Fetch, ImapError>> = match session.fetch("1:*", "FLAGS").await {
        Ok(e) => e.collect().await,
        Err(e) => return Err(MyError::Imap(e)),
    };

    let mut updated_uids: Vec<u32> = Vec::new();

    for fetch in &fetches {
        let fetch = match fetch {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                continue;
            }
        };

        let message_uid = match fetch.uid {
            Some(e) => e,
            None => continue,
        };

        updated_uids.push(message_uid);

        let flags = fetch.flags().collect::<Vec<_>>();

        let flags_str = flags_to_string(&flags);

        match conn::update_message_flags(
            database_conn,
            client,
            mailbox_path,
            message_uid,
            &flags_str,
        ) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    return Ok(updated_uids);
}
