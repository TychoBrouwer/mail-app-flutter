use async_imap::error::Error as ImapError;
use async_imap::types::{Fetch, Flag};
use futures::StreamExt;

use crate::inbox_client::{inbox_client::InboxClient, parse_message::flags_to_string};
use crate::my_error::MyError;

impl InboxClient {
    pub async fn modify_flags(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: u32,
        flags: &str,
        add: bool,
    ) -> Result<String, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        match session.select(mailbox_path).await {
            Ok(_) => {}
            Err(e) => match self.handle_disconnect(session_id, e).await {
                Ok(_) => {
                    return Box::pin(self.modify_flags(
                        session_id,
                        mailbox_path,
                        message_uid,
                        flags,
                        add,
                    ))
                    .await;
                }
                Err(e) => return Err(e),
            },
        };

        let mut query = if add { "+" } else { "-" }.to_string();

        query.push_str("FLAGS (");

        for (i, flag) in flags.split(",").enumerate() {
            query.push_str("\\");
            query.push_str(&flag);

            if i < flags.split(",").count() - 1 {
                query.push_str(" ");
            }
        }

        query.push_str(")");

        let fetches: Vec<Result<Fetch, ImapError>> =
            match session.uid_store(message_uid.to_string(), query).await {
                Ok(e) => e.collect().await,
                Err(e) => {
                    eprintln!("Error updating message flag");

                    return Err(MyError::Imap(e));
                }
            };

        let mut first_fetch: Option<Fetch> = None;
        for fetch in fetches {
            first_fetch = match fetch {
                Ok(first_fetch) => Some(first_fetch),
                Err(e) => {
                    eprintln!("Error updating message flag");

                    return Err(MyError::Imap(e));
                }
            };
        }

        let fetch = match first_fetch {
            Some(fetch) => fetch,
            None => {
                return Err(MyError::String("No fetches found".to_string()));
            }
        };

        let updated_flags = fetch.flags().collect::<Vec<_>>();

        return self.modify_flags_db(session_id, mailbox_path, message_uid, &updated_flags);
    }

    fn modify_flags_db(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: u32,
        flags: &[Flag],
    ) -> Result<String, MyError> {
        let flags_str = flags_to_string(flags);

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        match self.database_conn.update_message_flags(
            username,
            address,
            mailbox_path,
            message_uid,
            &flags_str,
        ) {
            Ok(_) => return Ok(flags_str),
            Err(e) => return Err(e),
        };
    }
}
