use std::{collections::HashMap, vec};

use crate::inbox_client::{
    inbox_client::InboxClient,
    parse_message::{flags_to_string, parse_message},
};
use crate::my_error::MyError;
use crate::types::{
    message::Message,
    sequence_set::{SequenceSet, StartEnd},
};

impl InboxClient {
    pub fn update_mailbox(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
    ) -> Result<String, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let (highest_seq, highest_seq_uid) =
            match self.get_highest_seq_imap(session_id, mailbox_path) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

        match self.get_highest_seq_db(session_id, mailbox_path, highest_seq_uid) {
            Ok(highest_seq_local) => {
                if highest_seq_local == highest_seq {
                    return Ok("[]".to_string());
                }
            }
            Err(_) => {}
        };

        let mut sequence_set = SequenceSet {
            nr_messages: None,
            start_end: Some(StartEnd { start: 1, end: 10 }),
            idx: None,
        };

        let mut changed_uids: Vec<u32> = Vec::new();

        loop {
            let new_changed_uids =
                match self.get_changed_message_uids(session_id, mailbox_path, &sequence_set) {
                    Ok(e) => e,
                    Err(e) => return Err(e),
                };

            changed_uids.extend(&new_changed_uids);

            if changed_uids.is_empty() {
                break;
            }

            match self.update_changed_messages(session_id, mailbox_path, &new_changed_uids) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            let end = sequence_set.start_end.unwrap().end;
            sequence_set.start_end = Some(StartEnd {
                start: end + 1,
                end: end + 10,
            });
        }

        let changed_flags_uids = match self.update_flags(session_id, mailbox_path) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        changed_uids.extend(&changed_flags_uids);

        let changed_uids_string = changed_uids
            .iter()
            .map(|uid| uid.to_string())
            .collect::<Vec<String>>()
            .join(",");

        return Ok(changed_uids_string);
    }

    fn get_highest_seq_imap(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
    ) -> Result<(u32, u32), MyError> {
        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        let mailbox = match session.select(mailbox_path) {
            Ok(m) => m,
            Err(e) => match self.handle_disconnect(session_id, e) {
                Ok(_) => {
                    return self.get_highest_seq_imap(session_id, mailbox_path);
                }
                Err(e) => return Err(e),
            },
        };

        let highest_seq = mailbox.exists;

        let fetches = match session.uid_fetch(format!("{}:{}", highest_seq, highest_seq), "UID") {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error fetching messages");

                return Err(MyError::Imap(e));
            }
        };

        let fetch = match fetches.first() {
            Some(e) => e,
            None => {
                return Err(MyError::String(
                    "Failed to get last message in inbox from imap".to_string(),
                ));
            }
        };

        let highest_seq_uid = match fetch.uid {
            Some(e) => e,
            None => {
                return Err(MyError::String(
                    "Failed to get last message in inbox from imap".to_string(),
                ));
            }
        };

        return Ok((highest_seq, highest_seq_uid));
    }

    fn get_highest_seq_db(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        highest_seq_uid: u32,
    ) -> Result<u32, MyError> {
        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        let messages = match self.database_conn.get_messages_with_uids(
            username,
            address,
            mailbox_path,
            &vec![highest_seq_uid],
        ) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };

        let message = messages.first();
        if message.is_some() {
            return Ok(message.unwrap().sequence_id);
        } else {
            return Err(MyError::String(
                "Failed to get last message in inbox from db".to_string(),
            ));
        }
    }

    fn get_changed_message_uids(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        sequence_set: &SequenceSet,
    ) -> Result<Vec<u32>, MyError> {
        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        let mailbox = match session.select(mailbox_path) {
            Ok(m) => m,
            Err(e) => match self.handle_disconnect(session_id, e) {
                Ok(_) => {
                    return self.get_changed_message_uids(session_id, mailbox_path, sequence_set);
                }
                Err(e) => {
                    return Err(e);
                }
            },
        };

        let sequence_set_string = match sequence_set.to_string(mailbox.exists, true) {
            Ok(e) => e,
            Err(e) => return Err(e),
        };

        let fetches = match session.fetch(sequence_set_string, "UID") {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error fetching messages");

                return Err(MyError::Imap(e));
            }
        };
        let messages_uids_imap: Vec<u32> = fetches.iter().filter_map(|fetch| fetch.uid).collect();

        let seq_to_uids_imap: HashMap<u32, u32> = fetches
            .iter()
            .filter_map(|fetch| fetch.uid.map(|uid| (uid, fetch.message)))
            .collect();

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        let messages = match self.database_conn.get_messages_with_uids(
            username,
            address,
            mailbox_path,
            &messages_uids_imap,
        ) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };

        let seq_to_uids_db: HashMap<u32, u32> = messages
            .iter()
            .map(|message| (message.sequence_id, message.uid))
            .collect();

        let changed_message_uids: Vec<u32> = seq_to_uids_imap
            .iter()
            .filter(|(seq, uid)| seq_to_uids_db.get(seq) != Some(uid))
            .map(|(_, uid)| *uid)
            .collect();

        return Ok(changed_message_uids);
    }

    fn update_changed_messages(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        changed_uids: &Vec<u32>,
    ) -> Result<(), MyError> {
        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        match session.select(mailbox_path) {
            Ok(m) => m,
            Err(e) => match self.handle_disconnect(session_id, e) {
                Ok(_) => {
                    return self.update_changed_messages(session_id, mailbox_path, changed_uids);
                }
                Err(e) => {
                    return Err(e);
                }
            },
        };

        let uid_set = changed_uids
            .iter()
            .map(|uid| uid.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let fetches = match session.uid_fetch(uid_set, "UID ENVELOPE BODY.PEEK[] FLAGS") {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error fetching messages");

                return Err(MyError::Imap(e));
            }
        };

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        let mut result: Vec<Message> = Vec::new();

        for fetch in &fetches {
            let message = match parse_message(fetch) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error parsing envelope: {:?}", e);
                    return Err(e);
                }
            };

            match self
                .database_conn
                .insert_message(username, address, mailbox_path, &message)
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error inserting message into db: {:?}", e);
                    return Err(e);
                }
            };

            result.push(message);
        }

        return Ok({});
    }

    fn update_flags(&mut self, session_id: usize, mailbox_path: &str) -> Result<Vec<u32>, MyError> {
        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        match session.select(mailbox_path) {
            Ok(m) => m,
            Err(e) => match self.handle_disconnect(session_id, e) {
                Ok(_) => {
                    return self.update_flags(session_id, mailbox_path);
                }
                Err(e) => return Err(e),
            },
        };

        let fetches = match session.fetch("1:*", "FLAGS") {
            Ok(e) => e,
            Err(e) => return Err(MyError::Imap(e)),
        };

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        let mut updated_uids: Vec<u32> = Vec::new();

        for fetch in &fetches {
            let message_uid = match fetch.uid {
                Some(e) => e,
                None => continue,
            };

            updated_uids.push(message_uid);

            let flags_str = flags_to_string(fetch.flags());

            match self.database_conn.update_message_flags(
                username,
                address,
                mailbox_path,
                message_uid,
                &flags_str,
            ) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        return Ok(updated_uids);
    }
}
