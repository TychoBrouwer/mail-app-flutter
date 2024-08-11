use crate::inbox_client::inbox_client::InboxClient;
use crate::my_error::MyError;

impl InboxClient {
    pub async fn move_message(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: u32,
        mailbox_path_dest: &str,
    ) -> Result<String, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(MyError::String("Session not found".to_string())),
        };

        match session.select(mailbox_path).await {
            Ok(_) => {}
            Err(e) => match self.handle_disconnect(session_id, e).await {
                Ok(_) => {
                    return Box::pin(self.move_message(
                        session_id,
                        mailbox_path,
                        message_uid,
                        mailbox_path_dest,
                    ))
                    .await;
                }
                Err(e) => return Err(e),
            },
        };

        match session
            .uid_mv(message_uid.to_string(), mailbox_path_dest)
            .await
        {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error moving message");
                return Err(MyError::Imap(e));
            }
        };

        return self.move_message_db(session_id, mailbox_path, message_uid, mailbox_path_dest);
    }

    fn move_message_db(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: u32,
        mailbox_path_dest: &str,
    ) -> Result<String, MyError> {
        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        match self.database_conn.move_message(
            username,
            address,
            mailbox_path,
            message_uid,
            mailbox_path_dest,
        ) {
            Ok(_) => return Ok(format!("\"{}\"", mailbox_path_dest)),
            Err(e) => return Err(e),
        };
    }
}
