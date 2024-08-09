use crate::inbox_client::inbox_client::InboxClient;
use crate::my_error::MyError;

impl InboxClient {
    pub fn move_message(
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

        match session.select(mailbox_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error selecting mailbox: {:?}", e);

                match e {
                    imap::Error::ConnectionLost => {
                        eprintln!("Reconnecting to IMAP server");

                        match self.connect_imap(session_id) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(e);
                            }
                        }

                        return self.move_message(
                            session_id,
                            mailbox_path,
                            message_uid,
                            mailbox_path_dest,
                        );
                    }
                    imap::Error::Io(_) => {
                        eprintln!("Reconnecting to IMAP server");

                        match self.connect_imap(session_id) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(e);
                            }
                        }

                        return self.move_message(
                            session_id,
                            mailbox_path,
                            message_uid,
                            mailbox_path_dest,
                        );
                    }
                    _ => {}
                }

                return Err(MyError::Imap(e));
            }
        };

        match session.uid_copy(message_uid.to_string(), mailbox_path_dest) {
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
            Ok(_) => return Ok(mailbox_path_dest.to_string()),
            Err(e) => return Err(e),
        };
    }
}
