mod database {
    pub mod db_connection;
}

pub mod inbox_client;

pub mod types {
    pub mod message;
    pub mod sequence_set;
    pub mod session;
}

mod http_server {
    pub mod handle_conn;
    pub mod http_server;
    pub mod params;
}

mod my_error;

use async_std::sync::{Arc, Mutex};

use crate::database::db_connection;
use crate::types::session::Session;

#[async_std::main]
async fn main() {
    let database_conn = match db_connection::new("mail.db").await {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {}", e),
    };

    match db_connection::initialise(&database_conn).await {
        Ok(_) => {}
        Err(e) => panic!("Error initialising database: {}", e),
    };

    let database_conn = Arc::new(Mutex::new(database_conn));

    let database_conn_2 = Arc::clone(&database_conn);
    let clients = match db_connection::get_connections(database_conn_2).await {
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

        match inbox_client::connect::connect(sessions, database_conn, clients, i).await {
            Ok(_) => {}
            Err(e) => eprintln!("Error connecting to IMAP stored in local database: {:?}", e),
        }
    }

    http_server::http_server::create_server(sessions, database_conn, clients).await;
}
