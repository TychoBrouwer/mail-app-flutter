use async_std::sync::Arc;
use async_std::{
    net::{TcpListener, TcpStream},
    prelude::*,
};
use futures::stream::StreamExt;
use rusqlite::Connection;

use crate::http_server::handle_conn;
use crate::types::tcp_session::TcpSessions;

pub async fn create_server(database_conn: &Connection, sessions: TcpSessions) {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();

    let _ = listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |tcpstream| {
            let sessions = Arc::clone(&sessions);

            async move {
                handle_connection(tcpstream.unwrap(), database_conn, sessions).await;
            }
        })
        .await;
}

async fn handle_connection(
    mut stream: TcpStream,
    database_conn: &Connection,
    sessions: TcpSessions,
) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let header = request.split("\r\n").next().unwrap_or("");
    let header_parts: Vec<&str> = header.split(" ").collect();

    let uri = header_parts.get(1).unwrap_or(&"");
    let uri_parts: Vec<&str> = uri.split("?").collect();

    let path: &str = uri_parts.get(0).unwrap_or(&"");
    let params: &str = uri_parts.get(1).unwrap_or(&"");

    let data = match path {
        "/login" => handle_conn::login(params, database_conn, sessions).await,
        "/logout" => handle_conn::logout(params, database_conn, sessions).await,
        "/get_sessions" => handle_conn::get_sessions(database_conn, sessions).await,
        "/get_mailboxes" => handle_conn::get_mailboxes(params, database_conn, sessions).await,
        "/get_messages_with_uids" => {
            handle_conn::get_messages_with_uids(params, database_conn, sessions).await
        }
        "/get_messages_sorted" => {
            handle_conn::get_messages_sorted(params, database_conn, sessions).await
        }
        "/update_mailbox" => handle_conn::update_mailbox(params, database_conn, sessions).await,
        "/modify_flags" => handle_conn::modify_flags(params, database_conn, sessions).await,
        "/move_message" => handle_conn::move_message(params, database_conn, sessions).await,
        _ => String::from("{\"success\": false, \"message\": \"Not Found\"}"),
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        data.len(),
        data
    );

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
