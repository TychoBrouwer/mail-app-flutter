use crate::inbox_client::inbox_client::InboxClient;
use crate::my_error::MyError;

impl InboxClient {
    pub fn get_messages_with_uids(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uids: &Vec<u32>,
    ) -> Result<String, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        let messages = match self.database_conn.get_messages_with_uids(
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

        let mut response = String::from("[");

        for (i, message) in messages.iter().rev().enumerate() {
            response.push_str(&message.to_string());

            if i < messages.len() - 1 {
                response.push_str(",");
            }
        }

        response.push_str("]");

        return Ok(response);
    }
}
