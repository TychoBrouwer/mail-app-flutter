use async_imap::error::Error as ImapError;
use async_imap::types::{Fetch, Flag};
use async_imap::Session;
use async_native_tls::TlsStream;
use async_std::net::TcpStream;
use futures::StreamExt;
use rusqlite::Connection;

use crate::database::conn;
use crate::inbox_client::inbox_client::handle_disconnect;
use crate::inbox_client::parse_message::flags_to_string;
use crate::my_error::MyError;
use crate::types::client::Client;

pub async fn modify_flags(
    mut session: Session<TlsStream<TcpStream>>,
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    message_uid: u32,
    flags: &str,
    add: bool,
) -> Result<String, MyError> {
    match session.select(mailbox_path).await {
        Ok(_) => {}
        Err(e) => match handle_disconnect(client, e).await {
            Ok(_) => {
                return Box::pin(modify_flags(
                    session,
                    database_conn,
                    client,
                    mailbox_path,
                    message_uid,
                    flags,
                    add,
                ))
                .await;
            }
            Err(e) => return Err(e),
        },
    };

    let mut query = if add { "+" } else { "-" }.to_string();

    query.push_str("FLAGS (");

    for (i, flag) in flags.split(",").enumerate() {
        query.push_str("\\");
        query.push_str(&flag);

        if i < flags.split(",").count() - 1 {
            query.push_str(" ");
        }
    }

    query.push_str(")");

    let fetches: Vec<Result<Fetch, ImapError>> =
        match session.uid_store(message_uid.to_string(), query).await {
            Ok(e) => e.collect().await,
            Err(e) => {
                eprintln!("Error updating message flag");

                return Err(MyError::Imap(e));
            }
        };

    let mut first_fetch: Option<Fetch> = None;
    for fetch in fetches {
        first_fetch = match fetch {
            Ok(first_fetch) => Some(first_fetch),
            Err(e) => {
                eprintln!("Error updating message flag");

                return Err(MyError::Imap(e));
            }
        };
    }

    let fetch = match first_fetch {
        Some(fetch) => fetch,
        None => {
            return Err(MyError::String("No fetches found".to_string()));
        }
    };

    let updated_flags = fetch.flags().collect::<Vec<_>>();

    return modify_flags_db(
        database_conn,
        client,
        mailbox_path,
        message_uid,
        &updated_flags,
    );
}

fn modify_flags_db(
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    message_uid: u32,
    flags: &[Flag],
) -> Result<String, MyError> {
    let flags_str = flags_to_string(flags);

    match conn::update_message_flags(database_conn, client, mailbox_path, message_uid, &flags_str) {
        Ok(_) => return Ok(flags_str),
        Err(e) => return Err(e),
    };
}
