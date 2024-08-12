use async_imap;
use async_imap::error::Error as ImapError;
use async_native_tls::TlsConnector;
use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};

use crate::database::db_connection;
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

pub mod get_mailboxes;
pub mod get_messages_sorted;
pub mod get_messages_with_uids;
pub mod modify_flags;
pub mod move_message;
pub mod parse_message;
pub mod update_mailbox;

pub async fn connect(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
    idx: usize,
) -> Result<usize, MyError> {
    let sessions_2 = Arc::clone(&sessions);
    let locked_sessions = sessions.lock().await;
    dbg!("locked sessions");

    let locked_clients = clients.lock().await;
    dbg!("locked clients");

    let client = &locked_clients[idx];

    let pos = locked_clients
        .iter()
        .position(|x| x.username == client.username && x.address == client.address);

    if locked_sessions.len() > pos.unwrap_or(0) {
        return Ok(pos.unwrap());
    }

    let idx = locked_sessions.len();

    drop(locked_sessions);

    match db_connection::insert_connection(database_conn, client.clone()).await {
        Ok(_) => {}
        Err(e) => eprintln!("Error inserting connection into database: {:?}", e),
    }

    drop(locked_clients);

    match connect_imap(sessions_2, clients).await {
        Ok(_) => {
            return Ok(idx);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub async fn connect_imap(
    sessions: Arc<Mutex<Vec<Session>>>,
    clients: Arc<Mutex<Vec<Client>>>,
) -> Result<(), MyError> {
    let locked_clients = clients.lock().await;
    dbg!("locked clients");

    let client = &locked_clients[locked_clients.len() - 1];

    let address = &client.address;
    let port = client.port;
    let username = &client.username;
    let password = &client.password;

    let tcp_stream = match TcpStream::connect((address.as_str(), port)).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error connecting to IMAP server: {}", e);
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
            eprintln!("Error connecting to IMAP server: {}", e);
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
            eprintln!("Error logging in: {:?}", e);
            let err = MyError::Imap(e.0, String::from("Error logging in"));
            err.log_error();

            return Err(err);
        }
    }
}

pub async fn logout_imap(
    sessions: Arc<Mutex<Vec<Session>>>,
    session_id: usize,
) -> Result<(), MyError> {
    let mut locked_sessions = sessions.lock().await;
    dbg!("locked sessions");

    if session_id >= locked_sessions.len() {
        let err = MyError::String(
            String::from("Session ID out of bounds"),
            String::from("Session not found"),
        );
        err.log_error();

        return Err(err);
    }

    let session = &mut locked_sessions[session_id];

    match session.logout().await {
        Ok(_) => {
            let mut locked_sessions = sessions.lock().await;
            dbg!("locked sessions");
            locked_sessions.remove(session_id);

            return Ok(());
        }
        Err(e) => {
            let err = MyError::Imap(e, String::from("Error logging out"));
            err.log_error();

            return Err(err);
        }
    }
}

pub async fn handle_disconnect(
    sessions: Arc<Mutex<Vec<Session>>>,
    clients: Arc<Mutex<Vec<Client>>>,
    e: ImapError,
) -> Result<(), MyError> {
    match e {
        ImapError::ConnectionLost => {
            eprintln!("Reconnecting to IMAP server");

            match connect_imap(sessions, clients).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }

            return Ok({});
        }
        ImapError::Io(_) => {
            eprintln!("Reconnecting to IMAP server");

            match connect_imap(sessions, clients).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }

            return Ok({});
        }
        _ => {}
    }

    let err = MyError::Imap(e, String::from("Error handling disconnect"));
    err.log_error();

    return Err(err);
}
