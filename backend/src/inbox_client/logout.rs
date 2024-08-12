use async_std::sync::{Arc, Mutex};

use crate::my_error::MyError;
use crate::types::session::Session;

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
