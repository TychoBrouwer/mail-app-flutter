use crate::inbox_client::{
    inbox_client::{InboxClient, SequenceSet},
    parse_message,
};

impl InboxClient {
    pub fn get_messages(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        sequence_set: SequenceSet,
    ) -> Result<String, String> {
        let message_uids = match self.get_messages_id(session_id, mailbox_path, &sequence_set) {
            Ok(ids) => ids,
            Err(e) => {
                eprintln!("Error getting message IDs: {:?}", e);
                return Err(String::from("{\"message\": \"Error getting message IDs\"}"));
            }
        };

        let messages_db = self.get_messages_db(session_id, mailbox_path, &message_uids);

        let messages = match messages_db {
            Ok(messages) => messages,
            Err(_) => {
                let messages_imap =
                    match self.get_messages_imap(session_id, mailbox_path, sequence_set) {
                        Ok(messages) => {
                            let username = &self.usernames[session_id];
                            let address = &self.addresses[session_id];

                            for message in &messages {
                                match self.database_conn.insert_message(
                                    username,
                                    address,
                                    mailbox_path,
                                    &message,
                                ) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!(
                                            "Error inserting message into local database: {:?}",
                                            e
                                        );

                                        return Err(String::from(
                                            "Error inserting message into local database",
                                        ));
                                    }
                                }
                            }

                            messages
                        }
                        Err(e) => {
                            eprintln!("Error getting message from IMAP: {:?}", e);
                            return Err(String::from("{\"message\": \"Error getting message\"}"));
                        }
                    };

                messages_imap
            }
        };

        let mut response = String::from("[");

        for (i, message) in messages.iter().enumerate() {
            response.push_str(&parse_message::message_to_string(&message));

            if i < messages.len() - 1 {
                response.push_str(",");
            }
        }

        response.push_str("]");

        return Ok(response);
    }

    fn get_messages_id(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        sequence_set: &SequenceSet,
    ) -> Result<Vec<u32>, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        session.select(mailbox_path).unwrap();

        let sequence_set_string: String = match InboxClient::sequence_set_to_string(&sequence_set) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error converting sequence set to string: {:?}", e);
                return Err(String::from("Error converting sequence set to string"));
            }
        };

        let message_uids = match session.fetch(&sequence_set_string, "UID") {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(String::from("Error fetching message"));
            }
        };

        let mut result: Vec<u32> = Vec::new();

        for fetch in &message_uids {
            let message_uid = match fetch.uid {
                Some(uid) => uid,
                None => continue,
            };

            result.push(message_uid);
        }

        return Ok(result);
    }

    fn get_messages_imap(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        sequence_set: SequenceSet,
    ) -> Result<Vec<parse_message::Message>, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        session.select(mailbox_path).unwrap();

        let sequence_set_string: String = match InboxClient::sequence_set_to_string(&sequence_set) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error converting sequence set to string: {:?}", e);
                return Err(String::from("Error converting sequence set to string"));
            }
        };

        let fetches = match session.fetch(&sequence_set_string, "(UID ENVELOPE BODY[])") {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(String::from("Error fetching message"));
            }
        };

        let mut result: Vec<parse_message::Message> = Vec::new();

        for fetch in &fetches {
            let message = match parse_message::parse_message(fetch) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error parsing envelope: {:?}", e);
                    return Err(String::from("Error parsing envelope"));
                }
            };

            result.push(message);
        }

        Ok(result)
    }

    fn get_messages_db(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uids: &Vec<u32>,
    ) -> Result<Vec<parse_message::Message>, String> {
        if session_id >= self.addresses.len() {
            return Err(String::from("Invalid session ID"));
        }

        let username = &self.usernames[session_id];
        let address = &self.addresses[session_id];

        let mut messages: Vec<parse_message::Message> = Vec::new();

        for message_uid in message_uids {
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

            messages.push(message);
        }

        return Ok(messages);
    }
}
