use async_imap::{self, Session};
use async_native_tls::{TlsConnector, TlsStream};
use async_std::net::TcpStream;
use rusqlite::Connection;

use crate::database::conn;
use crate::my_error::MyError;
use crate::types::client::Client;

// #[derive(Debug, Copy)]
// pub struct InboxClient {
//     pub sessions: Vec<Client>,
// }

// pub fn new(database_conn: DBConnection) -> InboxClient<'static> {
//     InboxClient {
//         sessions: Vec::new(),
//     }
// }

pub async fn connect(
    database_conn: &Connection,
    client: &Client,
) -> Result<Session<TlsStream<TcpStream>>, MyError> {
    match conn::insert_connection(database_conn, client) {
        Ok(_) => {}
        Err(e) => eprintln!("Error inserting connection into database: {:?}", e),
    }

    match connect_imap(client).await {
        Ok(conn) => {
            return Ok(conn);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn tls() -> TlsConnector {
    TlsConnector::new()
        .danger_accept_invalid_hostnames(true)
        .danger_accept_invalid_certs(true)
}

pub async fn connect_imap(client: &Client) -> Result<Session<TlsStream<TcpStream>>, MyError> {
    let address = &client.address;
    let port = client.port;
    let username = &client.username;
    let password = &client.password;

    let tcp_stream = match TcpStream::connect((address.as_str(), port)).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error connecting to IMAP server: {}", e);
            return Err(MyError::Io(e));
        }
    };

    let tls = tls();
    let tls_stream = match tls.connect(client.address.clone(), tcp_stream).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error establishing TLS connection: {}", e);
            return Err(MyError::Tls(e));
        }
    };

    let mut imap_client = async_imap::Client::new(tls_stream);
    let _greeting = match imap_client.read_response().await {
        Some(Ok(g)) => g,
        Some(Err(e)) => {
            eprintln!("Error reading greeting: {:?}", e);
            return Err(MyError::Io(e));
        }
        None => {
            return Err(MyError::String("No greeting received".to_string()));
        }
    };

    let session = match imap_client
        .login(username, password)
        .await
        .map_err(|(err, _client)| err)
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error logging in: {:?}", e);
            return Err(MyError::Imap(e));
        }
    };

    return Ok(session);
}

pub async fn logout_imap(mut session: Session<TlsStream<TcpStream>>) -> Result<(), MyError> {
    match session.logout().await {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error logging out: {:?}", e);
            return Err(MyError::Imap(e));
        }
    }
}

pub async fn handle_disconnect(
    client: &Client,
    e: async_imap::error::Error,
) -> Result<(), MyError> {
    eprintln!("IMAP communication error: {:?}", e);

    match e {
        async_imap::error::Error::ConnectionLost => {
            eprintln!("Reconnecting to IMAP server");

            // match connect_imap(session_id).await {
            //     Ok(_) => {}
            //     Err(e) => return Err(e),
            // }

            return Ok({});
        }
        async_imap::error::Error::Io(_) => {
            eprintln!("Reconnecting to IMAP server");

            // match connect_imap(session_id).await {
            //     Ok(_) => {}
            //     Err(e) => return Err(e),
            // }

            return Ok({});
        }
        _ => {}
    }

    return Err(MyError::Imap(e));
}
