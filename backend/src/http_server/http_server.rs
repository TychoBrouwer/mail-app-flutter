use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};
use async_std::{net::TcpListener, prelude::*};
use futures::stream::StreamExt;

use crate::http_server::handle_conn;
use crate::types::session::{Client, Session};

pub async fn create_server(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
) {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();

    let _ = listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |tcpstream| {
            let sessions = Arc::clone(&sessions);
            let database_conn = Arc::clone(&database_conn);
            let clients = Arc::clone(&clients);

            async move {
                handle_connection(tcpstream.unwrap(), sessions, database_conn, clients).await;
            }
        })
        .await;
}

async fn handle_connection(
    mut stream: TcpStream,
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
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
        "/login" => handle_conn::login(params, sessions, database_conn, clients).await,
        "/logout" => handle_conn::logout(params, sessions, clients).await,
        "/get_sessions" => handle_conn::get_sessions(clients).await,
        "/get_mailboxes" => {
            handle_conn::get_mailboxes(params, sessions, database_conn, clients).await
        }
        "/get_messages_with_uids" => {
            handle_conn::get_messages_with_uids(params, database_conn, clients).await
        }
        "/get_messages_sorted" => {
            handle_conn::get_messages_sorted(params, database_conn, clients).await
        }
        "/update_mailbox" => {
            handle_conn::update_mailbox(params, sessions, database_conn, clients).await
        }
        "/modify_flags" => {
            handle_conn::modify_flags(params, sessions, database_conn, clients).await
        }
        "/move_message" => {
            handle_conn::move_message(params, sessions, database_conn, clients).await
        }
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
