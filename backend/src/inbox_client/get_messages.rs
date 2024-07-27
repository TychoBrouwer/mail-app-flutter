use crate::inbox_client::inbox_client::{InboxClient, SequenceSet, StartEnd};

impl InboxClient {
    pub fn get_message_ids(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        sequence_set: SequenceSet,
    ) -> Result<String, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        session.select(mailbox_path).unwrap();

        let sequence_set_string: String = match sequence_set {
            SequenceSet {
                nr_messages: Some(nr_messages),
                start_end: None,
            } => {
                format!("1:{}", nr_messages)
            }
            SequenceSet {
                nr_messages: None,
                start_end: Some(StartEnd { start, end }),
            } => {
                if start > end {
                    return Err(String::from("Start must be less than or equal to end"));
                }

                format!("{}:{}", start, end)
            }
            _ => String::from("1:*"),
        };

        let message_uids = session.fetch(sequence_set_string, "UID").unwrap();

        let mut response = String::from("[");

        for (i, fetch) in message_uids.iter().enumerate() {
            let message_uid = match fetch.uid {
                Some(uid) => uid,
                None => continue,
            };

            response.push_str(&message_uid.to_string());
            if i < message_uids.len() - 1 {
                response.push_str(",");
            }
        }

        response.push_str("]");

        Ok(response)
    }
}
