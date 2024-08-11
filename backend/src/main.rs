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

use std::sync::{Arc, Mutex};

use crate::database::conn::DBConnection;
use crate::inbox_client::inbox_client::InboxClient;

#[async_std::main]
async fn main() {
    let mut database_conn = match DBConnection::new("mail.db") {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {}", e),
    };

    match database_conn.initialise() {
        Ok(_) => {}
        Err(e) => panic!("Error initialising database: {}", e),
    };

    let sessions = match database_conn.get_connections() {
        Ok(sessions) => sessions,
        Err(e) => panic!("Error getting connections: {}", e),
    };

    let inbox_client = Arc::new(Mutex::new(InboxClient::new(database_conn)));

    for session in sessions {
        let mut locked_inbox_client = inbox_client.lock().unwrap();
        match locked_inbox_client.connect(session).await {
            Ok(_) => {}
            Err(e) => eprintln!("Error connecting to IMAP stored in local database: {:?}", e),
        }
    }

    http_server::http_server::create_server(inbox_client).await;
    // websocket::websocket::create_server(&mut inbox_client);
}
