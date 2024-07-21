mod websocket {
    pub mod websocket;
    pub mod handle_conn;
    pub mod params;
}

mod inbox_client {
    pub mod inbox_client;
    pub mod parse_message;
}

fn main() {
    let mut inbox_client = inbox_client::inbox_client::InboxClient::new();

    websocket::websocket::create_server(&mut inbox_client);
}
