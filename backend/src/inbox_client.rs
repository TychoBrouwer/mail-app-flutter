use async_imap;
use async_imap::error::Error as ImapError;
use async_std::sync::{Arc, Mutex};

use crate::inbox_client;
use crate::my_error::MyError;
use crate::types::session::{Client, Session};

pub mod connect;
pub mod get_mailboxes;
pub mod get_messages_sorted;
pub mod get_messages_with_uids;
pub mod logout;
pub mod modify_flags;
pub mod move_message;
pub mod parse_message;
pub mod update_mailbox;

pub async fn handle_disconnect(
    sessions: Arc<Mutex<Vec<Session>>>,
    client: &Client,
    e: ImapError,
) -> Result<(), MyError> {
    match e {
        ImapError::ConnectionLost => {
            match inbox_client::connect::connect_imap(sessions, client).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }

            return Ok({});
        }
        ImapError::Io(_) => {
            match inbox_client::connect::connect_imap(sessions, client).await {
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
