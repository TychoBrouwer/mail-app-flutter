use async_std::sync::{Arc, Mutex};

use crate::types::session::Session;

pub mod database;
mod http_server {
    mod handle_conn;
    pub mod http_server;
    mod params;
}
pub mod inbox_client;
mod types {
    pub mod fetch_mode;
    pub mod message;
    pub mod sequence_set;
    pub mod session;
}
pub mod mime_parser {
    pub mod decode;
    pub mod parse_address;
    pub mod parse_time;
    pub mod parser;
}
mod my_error;

#[async_std::main]
async fn main() {
    let database_conn = match database::new("mail.db").await {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {}", e),
    };

    match database::initialise(&database_conn).await {
        Ok(_) => {}
        Err(e) => panic!("Error initialising database: {}", e),
    };

    let database_conn = Arc::new(Mutex::new(database_conn));

    let database_conn_2 = Arc::clone(&database_conn);
    let clients = match database::connections::get_all(database_conn_2).await {
        Ok(clients) => clients,
        Err(e) => panic!("Error getting connections: {}", e),
    };

    let nr_sessions = clients.len();

    let sessions: Arc<Mutex<Vec<Session>>> = Arc::new(Mutex::new(Vec::new()));
    let clients = Arc::new(Mutex::new(clients));

    for i in 0..nr_sessions {
        let sessions = Arc::clone(&sessions);
        let database_conn = Arc::clone(&database_conn);
        let clients = Arc::clone(&clients);

        let locked_clients = clients.lock().await;
        let client = locked_clients[i].clone();
        drop(locked_clients);

        match inbox_client::connect::connect(sessions, database_conn, clients, &client).await {
            Ok(_) => {}
            Err(e) => eprintln!("Error connecting to IMAP stored in local database: {:?}", e),
        }
    }

    http_server::http_server::create_server(sessions, database_conn, clients).await;
}
