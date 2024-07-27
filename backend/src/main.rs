mod database {
    pub mod conn;
}

mod inbox_client {
    pub mod get_mailboxes;
    pub mod get_message;
    pub mod get_messages;
    pub mod inbox_client;
    pub mod parse_message;
}

mod websocket {
    pub mod handle_conn;
    pub mod params;
    pub mod websocket;
}

fn main() {
    let mut database_conn = match database::conn::DBConnection::new("mail.db") {
        Ok(conn) => conn,
        Err(e) => {
            panic!("Error opening database: {}", e);
        }
    };

    match database_conn.initialise() {
        Ok(_) => {}
        Err(e) => {
            panic!("Error initialising database: {}", e);
        }
    };

    let connections = match database_conn.get_connections() {
        Ok(connections) => connections,
        Err(e) => {
            panic!("Error getting connections: {}", e);
        }
    };

    let mut inbox_client = inbox_client::inbox_client::InboxClient::new(database_conn);

    for connection in connections {
        match inbox_client.connect(connection) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error connecting to IMAP stored in local database: {:?}", e);
            }
        }
    }

    websocket::websocket::create_server(&mut inbox_client);
}
