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
                return Ok(parse_message::message_to_string(message));
            }
            Err(e) => {
                println!("Error getting message from local database: {:?}", e);

                let message_imap =
                    match self.get_message_imap(session_id, mailbox_path, message_uid) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Error getting message from IMAP: {:?}", e);
                            return Err(String::from("{\"message\": \"Error getting message\"}"));
                        }
                    };

                let message_envelope_imap =
                    match self.get_message_envelope_imap(session_id, mailbox_path, message_uid) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Error getting message envelope from IMAP: {:?}", e);
                            return Err(String::from(
                                "{\"message\": \"Error getting message envelope\"}",
                            ));
                        }
                    };

                let message = parse_message::message_merge(message_envelope_imap, message_imap);

                let username = &self.usernames[session_id];
                let address = &self.addresses[session_id];

                match self
                    .database_conn
                    .insert_message(username, address, mailbox_path, &message)
                {
                    Ok(_) => {
                        return Ok(parse_message::message_to_string(message));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    pub fn get_message_envelope_imap(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
    ) -> Result<parse_message::MessageBody, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        session.select(mailbox_path).unwrap();

        let envelope_fetch = match session.uid_fetch(message_uid.to_string(), "ENVELOPE") {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(String::from("Error fetching message"));
            }
        };

        let message_envelope = match envelope_fetch.first() {
            Some(e) => e,
            None => return Err(String::from("Message not found")),
        };

        match parse_message::parse_envelope(message_envelope, &message_uid) {
            Ok(message_body) => return Ok(message_body),
            Err(e) => {
                eprintln!("Error parsing envelope: {:?}", e);
                return Err(String::from("Error parsing envelope"));
            }
        };
    }

    pub fn get_message_imap(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
    ) -> Result<parse_message::MessageBody, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        session.select(&mailbox_path).unwrap();

        let messages = match session.uid_fetch(message_uid.to_string(), "BODY[]") {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(String::from("Error fetching message"));
            }
        };

        match messages.first() {
            Some(message) => {
                let message = match message.body() {
                    Some(m) => std::str::from_utf8(m).unwrap(),
                    None => return Err(String::from("Error getting message body")),
                };

                let message_body: parse_message::MessageBody =
                    parse_message::parse_message_body(message, message_uid);

                return Ok(message_body);
            }
            None => return Err(String::from("Message not found")),
        };
    }

    pub fn get_message_db(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
    ) -> Result<parse_message::MessageBody, String> {
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
                return Err(String::from("Error getting message"));
            }
        };

        return Ok(message);
    }
}
