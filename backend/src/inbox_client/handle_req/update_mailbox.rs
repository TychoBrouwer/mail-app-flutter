use std::{collections::HashMap, u32, vec};

use crate::inbox_client::{
    inbox_client::InboxClient,
    parse_message::{flags_to_string, parse_message},
};
use crate::my_error::MyError;
use crate::types::sequence_set::{SequenceSet, StartEnd};

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

        let mut changed_uids: Vec<u32> = Vec::new();
        let mut end = 0;

        loop {
            let mut start_end = StartEnd {
                start: end + 1,
                end: end + 50,
            };

            if start_end.start >= highest_seq {
                break;
            }
            if start_end.end > highest_seq {
                start_end.end = highest_seq;
            }

            end += 50;

            let sequence_set = SequenceSet {
                nr_messages: None,
                start_end: Some(start_end),
                idx: None,
            };

            let (moved_message_seq_to_uids, new_message_uids) =
                match self.get_changed_message_uids(session_id, mailbox_path, &sequence_set) {
                    Ok(e) => e,
                    Err(e) => return Err(e),
                };

            changed_uids.extend(
                &moved_message_seq_to_uids
                    .iter()
                    .map(|(seq, _)| *seq)
                    .collect::<Vec<u32>>(),
            );
            changed_uids.extend(&new_message_uids);

            if changed_uids.is_empty() {
                break;
            }

            if !new_message_uids.is_empty() {
                match self.get_new_messages(session_id, mailbox_path, &new_message_uids) {
                    Ok(e) => e,
                    Err(e) => return Err(e),
                };
            }

            if !moved_message_seq_to_uids.is_empty() {
                match self.update_moved_messeages(
                    session_id,
                    mailbox_path,
                    &moved_message_seq_to_uids,
                ) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                };
            }
        }

        let changed_flags_uids = match self.update_flags(session_id, mailbox_path) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        changed_uids.extend(&changed_flags_uids);

        let mut changed_uids_string = String::from("[");
        changed_uids_string.push_str(
            &changed_uids
                .iter()
                .map(|uid| uid.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        changed_uids_string.push_str("]");

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

        let fetches = match session.fetch(highest_seq.to_string(), "UID") {
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
            return Ok(message.unwrap().sequence_id.unwrap_or(u32::MAX));
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
    ) -> Result<(Vec<(u32, u32)>, Vec<u32>), MyError> {
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
            .filter_map(|fetch| fetch.uid.map(|uid| (fetch.message, uid)))
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
            .map(|message| (message.sequence_id.unwrap_or(u32::MAX), message.message_uid))
            .collect();

        let changed_message_uids: Vec<u32> = seq_to_uids_imap
            .iter()
            .filter(|(seq, uid)| seq_to_uids_db.get(seq) != Some(uid))
            .map(|(_, uid)| *uid)
            .collect();

        let new_messages_uids: Vec<u32> = changed_message_uids
            .iter()
            .filter(|uid| seq_to_uids_db.values().find(|v| **v == **uid).is_none())
            .map(|uid| *uid)
            .collect();

        let moved_message_seq_to_uids: Vec<(u32, u32)> = seq_to_uids_imap
            .iter()
            .filter(|(seq, uid)| seq_to_uids_db.get(seq) == Some(uid))
            .map(|(seq, uid)| (*seq, *uid))
            .collect();

        return Ok((moved_message_seq_to_uids, new_messages_uids));
    }

    fn get_new_messages(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        new_message_uids: &Vec<u32>,
    ) -> Result<(), MyError> {
        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        match session.select(mailbox_path) {
            Ok(m) => m,
            Err(e) => match self.handle_disconnect(session_id, e) {
                Ok(_) => {
                    return self.get_new_messages(session_id, mailbox_path, new_message_uids);
                }
                Err(e) => {
                    return Err(e);
                }
            },
        };

        let uid_set = new_message_uids
            .iter()
            .map(|uid| uid.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let fetches = match session.uid_fetch(&uid_set, "(UID ENVELOPE FLAGS BODY.PEEK[])") {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error fetching messages");

                return Err(MyError::Imap(e));
            }
        };

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        for fetch in &fetches {
            let message = match parse_message(fetch) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error parsing message: {:?}", e);
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
        }

        return Ok({});
    }

    fn update_moved_messeages(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        moved_message_seq_to_uids: &Vec<(u32, u32)>,
    ) -> Result<(), MyError> {
        for (sequence_id, message_uid) in moved_message_seq_to_uids {
            let username = &self.sessions[session_id].username;
            let address = &self.sessions[session_id].address;

            match self.database_conn.update_message_sequence_id(
                username,
                address,
                mailbox_path,
                *message_uid,
                *sequence_id,
            ) {
                Ok(_) => {}
                Err(e) => eprintln!("Error moving message: {:?}", e),
            }
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
