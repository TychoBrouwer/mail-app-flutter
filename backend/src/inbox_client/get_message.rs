use crate::inbox_client::{inbox_client::InboxClient, parse_message};

impl InboxClient {
    pub fn get_message(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
    ) -> Result<String, String> {
        let message_db = self.get_message_db(session_id, mailbox_path, message_uid);

        match message_db {
            Ok(message) => {
                return Ok(parse_message::message_to_string(&message));
            }
            Err(_) => {
                let message =
                    match self.get_message_imap(session_id, mailbox_path, message_uid) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Error getting message from IMAP: {:?}", e);
                            return Err(String::from("{\"message\": \"Error getting message\"}"));
                        }
                    };

                let username = &self.usernames[session_id];
                let address = &self.addresses[session_id];

                match self
                    .database_conn
                    .insert_message(username, address, mailbox_path, &message)
                {
                    Ok(_) => {
                        return Ok(parse_message::message_to_string(&message));
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
        message_uid: &u32,
    ) -> Result<parse_message::Message, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        session.select(mailbox_path).unwrap();

        let envelope_fetch = match session.uid_fetch(message_uid.to_string(), "(UID ENVELOPE BODY[])") {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(String::from("Error fetching message"));
            }
        };

        let fetch = match envelope_fetch.first() {
            Some(e) => e,
            None => return Err(String::from("Message not found")),
        };

        let message = match parse_message::parse_message(fetch) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error parsing envelope: {:?}", e);
                return Err(String::from("Error parsing envelope"));
            }
        };

        return Ok(message);
    }

    fn get_message_db(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
    ) -> Result<parse_message::Message, String> {
        if session_id >= self.addresses.len() {
            return Err(String::from("Invalid session ID"));
        }

        let username = &self.usernames[session_id];
        let address = &self.addresses[session_id];

        let message = match self.database_conn.get_message_with_uid(
            username,
            address,
            mailbox_path,
            message_uid,
        ) {
            Ok(m) => m,
            Err(_) => {
                return Err(String::from("Error getting message from local database"));
            }
        };

        return Ok(message);
    }
}
