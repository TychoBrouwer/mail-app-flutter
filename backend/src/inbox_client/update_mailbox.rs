use async_std::sync::{Arc, Mutex};
use std::{collections::HashMap, u32, vec};

use crate::database;
use crate::inbox_client;
use crate::my_error::MyError;
use crate::types::fetch_mode::FetchMode;
use crate::types::mailbox_changes::MailboxChanges;
use crate::types::sequence_set::{SequenceSet, StartEnd};
use crate::types::session::{Client, Session};

#[derive(Debug)]
struct ChangedSeqIdData {
    message_uid: u32,
    sequence_id_new: u32,
}

pub async fn update_mailbox(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    client: &Client,
    mailbox_path: &str,
    quick: bool,
) -> Result<MailboxChanges, MyError> {
    let sessions_2 = Arc::clone(&sessions);
    let (highest_seq, highest_seq_uid) =
        match get_highest_seq_imap(sessions_2, session_id, client, mailbox_path).await {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

    let mut mailbox_changes = MailboxChanges {
        new: vec![],
        changed: vec![],
        removed: vec![],
    };

    if quick {
        let database_conn_2 = Arc::clone(&database_conn);
        match get_highest_seq_db(database_conn_2, client, mailbox_path, highest_seq_uid).await {
            Ok(highest_seq_local) => {
                if highest_seq_local == highest_seq {
                    return Ok(mailbox_changes);
                }
            }
            Err(_) => {}
        };
    }

    let mut end = 0;
    let step_size = 20;

    loop {
        let mut start_end = StartEnd {
            start: end + 1,
            end: end + step_size,
        };

        if start_end.start >= highest_seq {
            break;
        }
        if start_end.end > highest_seq {
            start_end.end = highest_seq;
        }

        end += step_size;

        let sequence_set = SequenceSet {
            nr_messages: None,
            start_end: Some(start_end),
            idx: None,
        };

        let loop_mailbox_changes = match update_batch(
            Arc::clone(&sessions),
            session_id,
            Arc::clone(&database_conn),
            client,
            mailbox_path,
            &sequence_set,
        )
        .await
        {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

        mailbox_changes.new.extend(loop_mailbox_changes.new);
        mailbox_changes.removed.extend(loop_mailbox_changes.removed);

        if quick {
            break;
        }
    }

    if quick {
        let changed_uids =
            match update_flags(sessions, session_id, database_conn, client, mailbox_path).await {
                Ok(f) => f,
                Err(e) => return Err(e),
            };

        mailbox_changes.changed = changed_uids;
    }

    return Ok(mailbox_changes);
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

async fn update_batch(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    sequence_set: &SequenceSet,
) -> Result<MailboxChanges, MyError> {
    let sessions_2 = Arc::clone(&sessions);
    let database_conn_2 = Arc::clone(&database_conn);

    let (changed_seq_ids_data, new_message_uids, removed_messages_uids) = match get_changes(
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

    if changed_seq_ids_data.is_empty() && new_message_uids.is_empty() {
        return Ok(MailboxChanges {
            new: vec![],
            changed: vec![],
            removed: vec![],
        });
    }

    for message_uid in &removed_messages_uids {
        let database_conn_2 = Arc::clone(&database_conn);

        match database::message::remove(database_conn_2, client, mailbox_path, *message_uid).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        };
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

    for changed_seq_id in changed_seq_ids_data {
        let database_conn = Arc::clone(&database_conn);

        match database::message::update_sequence_id(
            database_conn,
            &client.username,
            &client.address,
            mailbox_path,
            changed_seq_id.message_uid,
            changed_seq_id.sequence_id_new,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => return Err(e),
        };
    }

    return Ok(MailboxChanges {
        new: new_message_uids,
        changed: vec![],
        removed: removed_messages_uids,
    });
}

async fn get_changes(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailbox_path: &str,
    sequence_set: &SequenceSet,
) -> Result<(Vec<ChangedSeqIdData>, Vec<u32>, Vec<u32>), MyError> {
    let fetches_imap = match inbox_client::messages::get_imap_with_seq(
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

    let uids_imap: Vec<u32> = fetches_imap.iter().map(|m| m.message_uid).collect();
    let uids_to_seq_imap: HashMap<u32, u32> = fetches_imap
        .iter()
        .map(|message| (message.message_uid, message.sequence_id))
        .collect();

    let database_conn_2 = Arc::clone(&database_conn);
    let messages_database = match database::messages::get_with_uids(
        database_conn_2,
        &client.username,
        &client.address,
        mailbox_path,
        &uids_imap,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let changed_seq_id_uids: Vec<ChangedSeqIdData> = messages_database
        .iter()
        .filter(|m| uids_to_seq_imap.get(&m.message_uid) != Some(&m.sequence_id))
        .map(|m| ChangedSeqIdData {
            message_uid: m.message_uid,
            sequence_id_new: *uids_to_seq_imap.get(&m.message_uid).unwrap(),
        })
        .collect();

    let seq_ids_to_remove: Vec<u32> = changed_seq_id_uids
        .iter()
        .map(|m| m.sequence_id_new)
        .collect();

    let messages_to_remove_database = match database::messages::get_with_seq_ids(
        database_conn,
        &client.username,
        &client.address,
        mailbox_path,
        &seq_ids_to_remove,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let removed_messages_uids = messages_to_remove_database
        .iter()
        .map(|m| m.message_uid)
        .collect();

    let new_messages_uids: Vec<u32> = uids_imap
        .iter()
        .filter(|uid| {
            messages_database
                .iter()
                .find(|m| m.message_uid == **uid)
                .is_none()
        })
        .map(|uid| *uid)
        .collect();

    return Ok((
        changed_seq_id_uids,
        new_messages_uids,
        removed_messages_uids,
    ));
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

    let flags_data = match database::messages::get_flags(
        Arc::clone(&database_conn),
        &client.username,
        &client.address,
        mailbox_path,
    )
    .await
    {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let changed_flags_uids: Vec<u32> = flags_data
        .iter()
        .filter_map(|f| {
            let message = messages.iter().find(|m| m.message_uid == f.0);
            if message.is_some() {
                if message.unwrap().flags != f.1 {
                    return Some(f.0);
                }
            }
            return None;
        })
        .collect();

    for changed_flags_uid in &changed_flags_uids {
        let database_conn_2 = Arc::clone(&database_conn);

        let message = messages
            .iter()
            .find(|m| m.message_uid == *changed_flags_uid)
            .unwrap();

        match database::message::update_flags(
            database_conn_2,
            &client.username,
            &client.address,
            mailbox_path,
            *changed_flags_uid,
            &message.flags,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    return Ok(changed_flags_uids);
}
