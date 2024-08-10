use crate::inbox_client::{inbox_client::InboxClient, parse_message};
use crate::my_error::MyError;

impl InboxClient {
    pub fn get_messages_sorted(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        start: u32,
        end: u32,
    ) -> Result<String, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        let messages = match self.database_conn.get_messages_sorted(
            username,
            address,
            mailbox_path,
            start,
            end,
        ) {
            Ok(m) => m,
            Err(e) => {
                return Err(e);
            }
        };

        let mut result = String::from("[");
        for (i, message) in messages.iter().enumerate() {
            result.push_str(&parse_message::message_to_string(&message));

            if i < messages.len() - 1 {
                result.push_str(",");
            }
        }
        result.push_str("]");

        return Ok(result);
    }
}
