use async_std::{
    net::{TcpListener, TcpStream},
    prelude::*,
};
use futures::stream::StreamExt;
use std::sync::{Arc, Mutex};

use crate::http_server::handle_conn;
use crate::inbox_client::inbox_client::InboxClient;

pub async fn create_server(inbox_client: Arc<Mutex<InboxClient>>) {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();

    let _ = listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |tcpstream| {
            let inbox_client = Arc::clone(&inbox_client);

            async move {
                handle_connection(tcpstream.unwrap(), inbox_client).await;
            }
        })
        .await;
}

async fn handle_connection(mut stream: TcpStream, inbox_client: Arc<Mutex<InboxClient>>) {
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
        "/login" => handle_conn::login(params, inbox_client),
        "/logout" => handle_conn::logout(params, inbox_client),
        "/get_sessions" => handle_conn::get_sessions(inbox_client),
        "/get_mailboxes" => handle_conn::get_mailboxes(params, inbox_client),
        "/get_messages_with_uids" => handle_conn::get_messages_with_uids(params, inbox_client),
        "/get_messages_sorted" => handle_conn::get_messages_sorted(params, inbox_client),
        "/update_mailbox" => handle_conn::update_mailbox(params, inbox_client),
        "/modify_flags" => handle_conn::modify_flags(params, inbox_client),
        "/move_message" => handle_conn::move_message(params, inbox_client),
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
