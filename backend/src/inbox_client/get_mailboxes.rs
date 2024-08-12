use async_std::stream::StreamExt;
use async_std::sync::{Arc, Mutex};

use async_imap::error::Error as ImapError;
use async_imap::types::Name;
use async_std::task;

use crate::database::db_connection;
use crate::inbox_client;
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

pub async fn get_mailboxes(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    session_id: usize,
    clients: Arc<Mutex<Vec<Client>>>,
) -> Result<String, MyError> {
    let database_conn_2 = Arc::clone(&database_conn);

    let locked_clients = clients.lock().await;
    dbg!("locked clients");

    let client = locked_clients[session_id].clone();

    let mailboxes_db = get_mailboxes_db(database_conn_2, &client);

    let mailboxes: Vec<String> = match mailboxes_db.await {
        Ok(mailboxes) => {
            drop(locked_clients);

            if !mailboxes.is_empty() {
                mailboxes
            } else {
                let mailboxes_imap: Result<Vec<String>, MyError> =
                    get_mailboxes_imap(sessions, session_id, clients).await;

                match mailboxes_imap {
                    Ok(mailboxes_imap) => mailboxes_imap,
                    Err(e) => {
                        eprintln!("Error getting mailboxes from IMAP: {:?}", e);
                        return Err(e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting mailboxes from local database: {:?}", e);

            return Err(e);
        }
    };

    let mut response = String::from("[");

    for (i, mailbox_path) in mailboxes.iter().enumerate() {
        let mailbox_path = mailbox_path.to_string();
        response.push_str(&format!("\"{}\"", mailbox_path));

        let database_conn = Arc::clone(&database_conn);
        let client = client.clone();

        task::spawn(async move {
            match db_connection::insert_mailbox(
                database_conn,
                &client.username,
                &client.address,
                &mailbox_path,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => eprintln!("Error inserting mailbox into local database: {:?}", e),
            }
        });

        if i < mailboxes.len() - 1 {
            response.push_str(",");
        }
    }

    response.push_str("]");

    return Ok(response);
}

async fn get_mailboxes_db(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
) -> Result<Vec<String>, MyError> {
    let mailboxes = match db_connection::get_mailboxes(
        database_conn,
        &client.username,
        &client.address,
    )
    .await
    {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error getting mailboxes: {:?}", e);
            return Err(e);
        }
    };

    return Ok(mailboxes);
}

async fn get_mailboxes_imap(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    clients: Arc<Mutex<Vec<Client>>>,
) -> Result<Vec<String>, MyError> {
    let sessions_2 = Arc::clone(&sessions);
    let clients_2 = Arc::clone(&clients);

    let mut sessions_lock = sessions.lock().await;
    dbg!("locked sessions");

    if session_id + 1 > sessions_lock.len() {
        let err = MyError::String(
            String::from("Out of bounds array access"),
            String::from("Invalid session ID"),
        );
        err.log_error();

        return Err(err);
    }

    let session = &mut sessions_lock[session_id];

    match session.capabilities().await {
        Ok(_) => {}
        Err(e) => {
            drop(sessions_lock);

            match inbox_client::handle_disconnect(sessions, clients, e).await {
                Ok(_) => {
                    return Box::pin(get_mailboxes_imap(sessions_2, session_id, clients_2)).await;
                }
                Err(e) => return Err(e),
            }
        }
    };

    let mailboxes: Vec<Result<Name, ImapError>> = match session.list(Some(""), Some("*")).await {
        Ok(m) => m.collect().await,
        Err(e) => {
            let err = MyError::Imap(e, String::from("Error getting mailboxes"));
            err.log_error();

            return Err(err);
        }
    };

    drop(sessions_lock);

    let mailboxes: Vec<String> = mailboxes
        .iter()
        .map(|mailbox| {
            let mailbox = match mailbox {
                Ok(m) => m.name(),
                Err(e) => {
                    eprintln!("Error getting mailbox: {:?}", e);
                    return "".to_string();
                }
            };

            mailbox.to_string()
        })
        .collect();

    return Ok(mailboxes);
}
