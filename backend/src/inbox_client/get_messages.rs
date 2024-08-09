use crate::inbox_client::{
    inbox_client::{InboxClient, SequenceSet},
    parse_message,
};

use super::my_error::MyError;

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
                return Err(format!("{{\"message\": \"{:?}\"}}", e));
            }
        };

        let messages_db_result = match self.get_messages_db(session_id, mailbox_path, &message_uids)
        {
            Ok(messages) => messages,
            Err(e) => return Err(format!("{{\"success\": false, \"message\": \"{:?}\"}}", e)),
        };

        let mut messages = messages_db_result.0;
        let failed_message_uids = messages_db_result.1;

        let failed_sequence_idx = failed_message_uids
            .iter()
            .map(|uid| message_uids.iter().position(|x| x == uid).unwrap())
            .collect::<Vec<usize>>();

        let offset = match &sequence_set.start_end {
            Some(start_end) => start_end.start - 1,
            None => 0,
        };

        let failed_sequence_set: SequenceSet = SequenceSet {
            nr_messages: None,
            start_end: None,
            idx: Some(failed_sequence_idx.iter().map(|x| x + offset + 1).collect()),
        };

        match failed_message_uids.len() {
            0 => {}
            _ => match self.get_messages_imap(session_id, mailbox_path, failed_sequence_set) {
                Ok(messages_imap) => {
                    let username = &self.sessions[session_id].username;
                    let address = &self.sessions[session_id].address;

                    for message_imap in &messages_imap {
                        match self.database_conn.insert_message(
                            username,
                            address,
                            mailbox_path,
                            &message_imap,
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Error inserting message into local database: {:?}", e);

                                return Err(format!(
                                    "{{\"success\": false, \"message\": \"{:?}\"}}",
                                    e
                                ));
                            }
                        }
                    }

                    messages.extend(messages_imap);
                }
                Err(e) => {
                    eprintln!("Error getting message from IMAP: {:?}", e);
                    return Err(format!("{{\"success\": false, \"message\": \"{:?}\"}}", e));
                }
            },
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
    ) -> Result<Vec<u32>, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        match session.select(mailbox_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error selecting mailbox: {:?}", e);
                return Err(MyError::Imap(e));
            }
        }

        let sequence_set_string: String = match InboxClient::sequence_set_to_string(&sequence_set) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error converting sequence set to string: {:?}", e);
                return Err(e);
            }
        };

        let message_uids = match session.fetch(&sequence_set_string, "UID") {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(MyError::Imap(e));
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
    ) -> Result<Vec<parse_message::Message>, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        match session.select(mailbox_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error selecting mailbox: {:?}", e);
                return Err(MyError::Imap(e));
            }
        }

        let sequence_set_string: String = match InboxClient::sequence_set_to_string(&sequence_set) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error converting sequence set to string: {:?}", e);
                return Err(e);
            }
        };

        let fetches = match session.fetch(&sequence_set_string, "(UID ENVELOPE BODY.PEEK[] FLAGS)")
        {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(MyError::Imap(e));
            }
        };

        let mut result: Vec<parse_message::Message> = Vec::new();

        for fetch in &fetches {
            let message = match parse_message::parse_message(fetch) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error parsing envelope: {:?}", e);
                    return Err(e);
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
    ) -> Result<(Vec<parse_message::Message>, Vec<u32>), MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        let messages = match self.database_conn.get_messages_with_uid(
            username,
            address,
            mailbox_path,
            message_uids,
        ) {
            Ok(m) => m,
            Err(e) => {
                return Err(e);
            }
        };

        let failed_message_uids = message_uids
            .iter()
            .filter(|uid| !messages.iter().any(|m| m.uid == **uid))
            .map(|x| *x)
            .collect();

        return Ok((messages, failed_message_uids));
    }
}
