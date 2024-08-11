mod database {
    pub mod conn;
}

mod inbox_client {
    mod handle_req {
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

mod types {
    pub mod client;
    pub mod message;
    pub mod sequence_set;
    pub mod tcp_session;
}

mod http_server {
    pub mod handle_conn;
    pub mod http_server;
    pub mod params;
}

mod my_error;

use async_std::sync::{Arc, Mutex};
use database::conn;
use types::tcp_session::TcpSessions;

#[async_std::main]
async fn main() {
    let database_conn = match conn::new("mail.db") {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {}", e),
    };

    match conn::initialise(&database_conn) {
        Ok(_) => {}
        Err(e) => panic!("Error initialising database: {}", e),
    };

    let clients = match conn::get_connections(&database_conn) {
        Ok(clients) => clients,
        Err(e) => panic!("Error getting connections: {}", e),
    };

    let mut sessions = Vec::new();
    for client in clients {
        let session = inbox_client::inbox_client::connect_imap(&client).await;

        match session {
            Ok(s) => sessions.push(s),
            Err(e) => eprintln!("Error connecting to IMAP: {:?}", e),
        }
    }

    let sessions: TcpSessions = Arc::new(Mutex::new(sessions));

    http_server::http_server::create_server(&database_conn, sessions).await;
}
