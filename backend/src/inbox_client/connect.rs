use async_imap;
use async_imap::error::Error as ImapError;
use async_native_tls::TlsConnector;
use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};

use crate::database;
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

pub async fn connect(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
    client_add: &Client,
) -> Result<usize, MyError> {
    let idx_db = match database(database_conn, clients, client_add).await {
        Ok(idx) => idx,
        Err(e) => return Err(e),
    };

    let idx_imap = match imap(sessions, &client_add).await {
        Ok(_) => idx_db,
        Err(e) => return Err(e),
    };

    assert!(idx_db == idx_imap);

    return Ok(idx_db);
}

async fn database(
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
    client_add: &Client,
) -> Result<usize, MyError> {
    let mut locked_clients = clients.lock().await;

    let pos = locked_clients
        .iter()
        .position(|x| x.username == client_add.username && x.address == client_add.address);

    if locked_clients.len() > pos.unwrap_or(0) {
        return Ok(pos.unwrap());
    }

    locked_clients.push(client_add.clone());
    let idx = locked_clients.len() - 1;

    drop(locked_clients);

    match database::connections::insert(database_conn, client_add).await {
        Ok(_) => {}
        Err(e) => {
            let mut locked_clients = clients.lock().await;
            locked_clients.remove(idx);
            drop(locked_clients);

            return Err(e);
        }
    }

    return Ok(idx);
}

pub async fn handle_disconnect(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
    client: &Client,
    e: ImapError,
) -> Result<(), MyError> {
    let mut locked_sessions = sessions.lock().await;

    match locked_sessions[session_id].close().await {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Imap(e, String::from("Error closing session"));
            err.log_error();

            return Err(err);
        }
    }
    drop(locked_sessions);

    match e {
        ImapError::ConnectionLost => {
            match imap(sessions, client).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }

            return Ok(());
        }
        ImapError::Io(_) => {
            match imap(sessions, client).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }

            return Ok(());
        }
        _ => {}
    }

    let err = MyError::Imap(e, String::from("Error handling disconnect"));
    err.log_error();

    return Err(err);
}

async fn imap(sessions: Arc<Mutex<Vec<Session>>>, client: &Client) -> Result<(), MyError> {
    let address = &client.address;
    let port = client.port;
    let username = &client.username;
    let password = &client.password;

    let tcp_stream = match TcpStream::connect((address.as_str(), port)).await {
        Ok(s) => s,
        Err(e) => {
            let err = MyError::Io(e, String::from("Error connecting to IMAP server"));
            err.log_error();

            return Err(err);
        }
    };

    let tls = TlsConnector::new()
        .danger_accept_invalid_hostnames(true)
        .danger_accept_invalid_certs(true);
    let tls_stream = match tls.connect(address, tcp_stream).await {
        Ok(s) => s,
        Err(e) => {
            let err = MyError::Tls(e, String::from("Error connecting to IMAP server"));
            err.log_error();

            return Err(err);
        }
    };

    let mut client = async_imap::Client::new(tls_stream);
    let _greeting = client.read_response().await;

    match client.login(username, password).await {
        Ok(session) => {
            let mut locked_sessions = sessions.lock().await;
            locked_sessions.push(session);

            return Ok(());
        }
        Err(e) => {
            let err = MyError::Imap(e.0, String::from("Error logging in"));
            err.log_error();

            return Err(err);
        }
    }
}
