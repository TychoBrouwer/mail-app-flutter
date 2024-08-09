use crate::inbox_client::{inbox_client::InboxClient, my_error::MyError};

impl InboxClient {
    pub fn get_mailboxes(&mut self, session_id: usize) -> Result<String, MyError> {
        let mailboxes_db = self.get_mailboxes_db(session_id);

        let mailboxes: Vec<String> = match mailboxes_db {
            Ok(mailboxes) => {
                if mailboxes.len() > 0 {
                    mailboxes
                } else {
                    let mailboxes_imap: Result<Vec<String>, MyError> =
                        self.get_mailboxes_imap(session_id);

                    match mailboxes_imap {
                        Ok(mailboxes_imap) => mailboxes_imap,
                        Err(e) => {
                            eprintln!("Error getting mailboxes from IMAP: {:?}", e);
                            return Err(e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error getting mailboxes from local database: {:?}", e);

                return Err(e);
            }
        };

        let mut response = String::from("[");

        for (i, mailbox_path) in mailboxes.iter().enumerate() {
            response.push_str(&format!("\"{}\"", mailbox_path));

            let username = &self.sessions[session_id].username;
            let address = &self.sessions[session_id].address;

            match self
                .database_conn
                .insert_mailbox(username, address, mailbox_path)
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error inserting mailbox into local database: {:?}", e);
                }
            }

            if i < mailboxes.len() - 1 {
                response.push_str(",");
            }
        }

        response.push_str("]");

        return Ok(response);
    }

    fn get_mailboxes_db(&mut self, session_id: usize) -> Result<Vec<String>, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;

        let mailboxes = match self.database_conn.get_mailboxes(username, address) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error getting mailboxes: {:?}", e);
                return Err(e);
            }
        };

        return Ok(mailboxes);
    }

    fn get_mailboxes_imap(&mut self, session_id: usize) -> Result<Vec<String>, MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Invalid session ID".to_string()));
        }

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => {
                return Err(MyError::String("Session not found".to_string()));
            }
        };

        let mailboxes = match session.list(Some(""), Some("*")) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error getting mailboxes from IMAP: {:?}", e);

                match e {
                    imap::Error::ConnectionLost => {
                        eprintln!("Reconnecting to IMAP server");

                        match self.connect_imap(session_id) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(e);
                            }
                        }

                        return self.get_mailboxes_imap(session_id);
                    }
                    imap::Error::Io(_) => {
                        eprintln!("Reconnecting to IMAP server");

                        match self.connect_imap(session_id) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(e);
                            }
                        }

                        return self.get_mailboxes_imap(session_id);
                    }

                    _ => {}
                }

                return Err(MyError::Imap(e));
            }
        };

        let mailboxes: Vec<String> = mailboxes
            .iter()
            .map(|mailbox| {
                let mailbox = mailbox.name();

                mailbox.to_string()
            })
            .collect();

        return Ok(mailboxes);
    }
}
