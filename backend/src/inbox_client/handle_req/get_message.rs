use crate::inbox_client::{
    inbox_client::InboxClient,
    parse_message::{message_to_string, parse_message, Message},
};
use crate::my_error::MyError;

impl InboxClient {
    pub fn get_message(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: u32,
    ) -> Result<String, MyError> {
        let message_db = self.get_message_db(session_id, mailbox_path, message_uid);

        match message_db {
            Ok(message) => {
                return Ok(message_to_string(&message));
            }
            Err(_) => {
                let message = match self.get_message_imap(session_id, mailbox_path, message_uid) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("Error getting message from IMAP: {:?}", e);
                        return Err(e);
                    }
                };

                let username = &self.sessions[session_id].username;
                let address = &self.sessions[session_id].address;

                match self
                    .database_conn
                    .insert_message(username, address, mailbox_path, &message)
                {
                    Ok(_) => {
                        return Ok(message_to_string(&message));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    fn get_message_imap(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: u32,
    ) -> Result<Message, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        match session.select(mailbox_path) {
            Ok(m) => m,
            Err(e) => match self.handle_disconnect(session_id, e) {
                Ok(_) => {
                    return self.get_message_imap(session_id, mailbox_path, message_uid);
                }
                Err(e) => {
                    return Err(e);
                }
            },
        };

        let fetches =
            match session.uid_fetch(message_uid.to_string(), "(UID ENVELOPE BODY.PEEK[] FLAGS)") {
                Ok(fetch) => fetch,
                Err(e) => {
                    eprintln!("Error fetching message: {:?}", e);
                    return Err(MyError::Imap(e));
                }
            };

        let fetch = match fetches.first() {
            Some(e) => e,
            None => return Err(MyError::String("Message not found".to_string())),
        };

        let message = match parse_message(fetch) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error parsing envelope: {:?}", e);
                return Err(e);
            }
        };

        return Ok(message);
    }

    fn get_message_db(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: u32,
    ) -> Result<Message, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        let message = match self.database_conn.get_message_with_uid(
            username,
            address,
            mailbox_path,
            message_uid,
        ) {
            Ok(m) => m,
            Err(e) => {
                return Err(e);
            }
        };

        return Ok(message);
    }
}
