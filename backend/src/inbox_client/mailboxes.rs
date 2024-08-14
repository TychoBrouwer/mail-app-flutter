use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};
use async_std::task;
use futures::future::join_all;

use crate::database;
use crate::inbox_client;
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

pub async fn update(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    client: &Client,
) -> Result<Vec<String>, MyError> {
    let mailboxes = match get_imap(sessions, session_id, client).await {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    match store_database(database_conn, client, &mailboxes).await {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    return Ok(mailboxes);
}

pub async fn get_database(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
) -> Result<Vec<String>, MyError> {
    let mailboxes =
        match database::mailbox::get(database_conn, &client.username, &client.address).await {
            Ok(m) => m,
            Err(e) => {
                return Err(e);
            }
        };

    return Ok(mailboxes);
}

async fn get_imap(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    client: &Client,
) -> Result<Vec<String>, MyError> {
    let sessions_2 = Arc::clone(&sessions);
    let sessions_3 = Arc::clone(&sessions);

    let mut sessions_lock = sessions.lock().await;

    if session_id + 1 > sessions_lock.len() {
        let err = MyError::String(
            String::from("Out of bounds array access"),
            String::from("Invalid session ID"),
        );
        err.log_error();

        return Err(err);
    }

    let session = &mut sessions_lock[session_id];

    let mailboxes: Vec<_> = match session.list(Some(""), Some("*")).await {
        Ok(m) => m.collect().await,
        Err(e) => match inbox_client::connect::handle_disconnect(sessions_3, session_id, client, e)
            .await
        {
            Ok(_) => {
                return Box::pin(get_imap(sessions_2, session_id, client)).await;
            }
            Err(e) => return Err(e),
        },
    };

    drop(sessions_lock);

    let mailboxes: Vec<String> = mailboxes
        .iter()
        .map(|mailbox| {
            let mailbox = match mailbox {
                Ok(m) => m.name(),
                Err(_) => {
                    return "".to_string();
                }
            };

            mailbox.to_string()
        })
        .collect();

    return Ok(mailboxes);
}

async fn store_database(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    mailboxes: &Vec<String>,
) -> Result<(), MyError> {
    let mut tasks = vec![];

    for mailbox_path in mailboxes {
        let mailbox_path = mailbox_path.to_string();

        let database_conn = Arc::clone(&database_conn);
        let client = client.clone();

        tasks.push(task::spawn(async move {
            match database::mailbox::insert(
                database_conn,
                &client.username,
                &client.address,
                &mailbox_path,
            )
            .await
            {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            }
        }));
    }

    join_all(tasks).await;

    return Ok(());
}
