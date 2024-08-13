use async_std::sync::{Arc, Mutex};
use rusqlite::Connection;

use crate::database;
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

pub async fn logout(
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    client: &Client,
    session_id: usize,
) -> Result<(), MyError> {
    match imap(Arc::clone(&sessions), session_id).await {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    match database(database_conn, client).await {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    return Ok(());
}

async fn imap(sessions: Arc<Mutex<Vec<Session>>>, session_id: usize) -> Result<(), MyError> {
    let mut locked_sessions = sessions.lock().await;
    
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

async fn database(database_conn: Arc<Mutex<Connection>>, client: &Client) -> Result<(), MyError> {
    match database::connections::remove(database_conn, client).await {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    return Ok(());
}
