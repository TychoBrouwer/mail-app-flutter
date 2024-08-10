use imap::types::Flag;

use crate::inbox_client::{inbox_client::InboxClient, parse_message::flags_to_string};
use crate::my_error::MyError;

impl InboxClient {
    pub fn modify_flags(
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

        match session.select(mailbox_path) {
            Ok(_) => {}
            Err(e) => match self.handle_disconnect(session_id, e) {
                Ok(_) => {
                    return self.modify_flags(session_id, mailbox_path, message_uid, flags, add);
                }
                Err(e) => {
                    return Err(e);
                }
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

        let fetches = match session.uid_store(message_uid.to_string(), query) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error updating message flag");

                return Err(MyError::Imap(e));
            }
        };

        let fetch = match fetches.first() {
            Some(e) => e,
            None => {
                return Err(MyError::String("No messages updated".to_string()));
            }
        };

        let updated_flags = fetch.flags();

        return self.modify_flags_db(session_id, mailbox_path, message_uid, updated_flags);
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
