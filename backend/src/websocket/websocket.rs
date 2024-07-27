use crate::{inbox_client::inbox_client, websocket::handle_conn};

use std::net::TcpListener;
use tungstenite::accept;

pub fn create_server(inbox_client: &mut inbox_client::InboxClient) {
    let server = TcpListener::bind("localhost:9001").unwrap();

    for stream in server.incoming() {
        let mut websocket = accept(stream.unwrap()).unwrap();

        loop {
            let msg: tungstenite::Message = match websocket.read() {
                Ok(msg) => msg,
                Err(e) => {
                    if matches!(e, tungstenite::Error::ConnectionClosed) {
                        break;
                    }

                    eprintln!("Error reading from websocket: {:?}", e);
                    break;
                }
            };

            // We do not want to send back ping/pong messages.
            if msg.is_text() {
                let msg: String = match msg {
                    tungstenite::Message::Text(msg) => msg,
                    _ => continue,
                };

                let result = handle_connection(&msg, inbox_client);

                websocket.send(tungstenite::Message::Text(result)).unwrap();
            }
        }
    }
}

fn handle_connection(msg: &str, inbox_client: &mut inbox_client::InboxClient) -> String {
    let uri_parts: Vec<&str> = msg.split("\r\n").collect();

    if uri_parts.len() != 2 {
        return String::from("{\"message\": \"Bad Request\"}");
    };

    let request = uri_parts[0];
    let data = uri_parts[1];

    match request {
        "/imap/login" => handle_conn::login(data, inbox_client),
        "/imap/logout" => handle_conn::logout(data, inbox_client),
        "/imap/message" => handle_conn::message(data, inbox_client),
        "/imap/mailboxes" => handle_conn::mailboxes(data, inbox_client),
        "/imap/message_envelopes" => handle_conn::message_envelopes(data, inbox_client),
        _ => String::from("{\"message\": \"Not Found\"}"),
    }
}
