use crate::inbox_client::{inbox_client::InboxClient, parse_message};
use crate::my_error::MyError;

impl InboxClient {
    pub fn update_mailbox(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
    ) -> Result<String, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        return Ok("".to_string());
    }
}
