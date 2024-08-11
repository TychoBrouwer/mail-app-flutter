use async_imap::error::Error as ImapError;
use async_imap::{types::Name, Session};
use async_native_tls::TlsStream;
use async_std::net::TcpStream;
use futures::StreamExt;
use rusqlite::Connection;

use crate::database::conn;
use crate::inbox_client::inbox_client::handle_disconnect;
use crate::my_error::MyError;
use crate::types::client::Client;

pub async fn get_mailboxes(
    session: Session<TlsStream<TcpStream>>,
    database_conn: &Connection,
    client: &Client,
) -> Result<String, MyError> {
    let mailboxes_db = get_mailboxes_db(&database_conn, &client);

    let mailboxes: Vec<String> = match mailboxes_db {
        Ok(mailboxes) => {
            if !mailboxes.is_empty() {
                mailboxes
            } else {
                let mailboxes_imap: Result<Vec<String>, MyError> =
                    get_mailboxes_imap(session, client).await;

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
        response.push_str(&format!("\"{}\"", mailbox_path));

        match conn::insert_mailbox(&database_conn, &client, mailbox_path) {
            Ok(_) => {}
            Err(e) => eprintln!("Error inserting mailbox into local database: {:?}", e),
        }

        if i < mailboxes.len() - 1 {
            response.push_str(",");
        }
    }

    response.push_str("]");

    return Ok(response);
}

fn get_mailboxes_db(conn: &Connection, client: &Client) -> Result<Vec<String>, MyError> {
    let mailboxes = match conn::get_mailboxes(conn, client) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error getting mailboxes: {:?}", e);
            return Err(e);
        }
    };

    return Ok(mailboxes);
}

async fn get_mailboxes_imap(
    mut session: Session<TlsStream<TcpStream>>,
    client: &Client,
) -> Result<Vec<String>, MyError> {
    match session.capabilities().await {
        Ok(_) => {}
        Err(e) => match handle_disconnect(client, e).await {
            Ok(_) => {
                return Box::pin(get_mailboxes_imap(session, client)).await;
            }
            Err(e) => return Err(e),
        },
    };

    let mailboxes: Vec<Result<Name, ImapError>> = match session.list(Some(""), Some("*")).await {
        Ok(m) => m.collect().await,
        Err(e) => return Err(MyError::Imap(e)),
    };

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
