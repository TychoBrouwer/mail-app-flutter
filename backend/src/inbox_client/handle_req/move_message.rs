use async_imap::Session;
use async_native_tls::TlsStream;
use async_std::net::TcpStream;
use rusqlite::Connection;

use crate::database::conn;
use crate::inbox_client::inbox_client::handle_disconnect;
use crate::my_error::MyError;
use crate::types::client::Client;

pub async fn move_message(
    mut session: Session<TlsStream<TcpStream>>,
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    message_uid: u32,
    mailbox_path_dest: &str,
) -> Result<String, MyError> {
    match session.select(mailbox_path).await {
        Ok(_) => {}
        Err(e) => match handle_disconnect(client, e).await {
            Ok(_) => {
                return Box::pin(move_message(
                    session,
                    database_conn,
                    client,
                    mailbox_path,
                    message_uid,
                    mailbox_path_dest,
                ))
                .await;
            }
            Err(e) => return Err(e),
        },
    };

    match session
        .uid_mv(message_uid.to_string(), mailbox_path_dest)
        .await
    {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error moving message");
            return Err(MyError::Imap(e));
        }
    };

    return move_message_db(
        database_conn,
        client,
        mailbox_path,
        message_uid,
        mailbox_path_dest,
    );
}

fn move_message_db(
    database_conn: &Connection,
    client: &Client,
    mailbox_path: &str,
    message_uid: u32,
    mailbox_path_dest: &str,
) -> Result<String, MyError> {
    match conn::move_message(
        database_conn,
        client,
        mailbox_path,
        message_uid,
        mailbox_path_dest,
    ) {
        Ok(_) => return Ok(format!("\"{}\"", mailbox_path_dest)),
        Err(e) => return Err(e),
    };
}
