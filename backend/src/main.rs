mod database {
    pub mod conn;
}

mod inbox_client {
    pub mod handle_req {
        pub mod get_mailboxes;
        pub mod get_messages_sorted;
        pub mod get_messages_with_uids;
        pub mod modify_flags;
        pub mod move_message;
        pub mod update_mailbox;
    }
    pub mod inbox_client;
    pub mod parse_message;
}

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

use crate::database::conn::DBConnection;
use crate::inbox_client::inbox_client::InboxClient;
use crate::types::session::Session;

#[async_std::main]
async fn main() {
    let database_conn = match DBConnection::new("mail.db").await {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {}", e),
    };

    match DBConnection::initialise(&database_conn).await {
        Ok(_) => {}
        Err(e) => panic!("Error initialising database: {}", e),
    };

    let database_conn = Arc::new(Mutex::new(database_conn));

    let database_conn_2 = Arc::clone(&database_conn);
    let clients = match DBConnection::get_connections(database_conn_2).await {
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

        match InboxClient::connect(sessions, database_conn, clients, i).await {
            Ok(_) => {}
            Err(e) => eprintln!("Error connecting to IMAP stored in local database: {:?}", e),
        }
    }

    http_server::http_server::create_server(sessions, database_conn, clients).await;
    // websocket::websocket::create_server(&mut inbox_client);
}
