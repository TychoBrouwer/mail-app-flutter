use crate::inbox_client::inbox_client::InboxClient;

impl InboxClient {
    pub fn modify_flag(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
        flags: &str,
        add: bool,
    ) -> Result<String, String> { 
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => return Err(String::from("Session not found")),
        };

        match session.select(mailbox_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error selecting mailbox: {:?}", e);
                return Err(String::from("Error selecting mailbox"));
            }
        }

        let mut query = if add { "+" } else { "-" }.to_string();
        
        query.push_str("FLAGS.SILENT (");
        
        for (i, flag) in flags.split(",").enumerate() {
            query.push_str("\\");
            query.push_str(&flag);

            if i < flags.split(",").count() - 1 {
                query.push_str(",");
            }
        };

        query.push_str(")");

        dbg!(&query);

        match session.uid_store(message_uid.to_string(), query) {
            Ok(e) => e,
            Err(_) => {
                eprintln!("Error updating message flag");

                return Err(String::from("Error updating message flag"));
            }
        };

        let username = &self.sessions[session_id].username;
        let address = &self.sessions[session_id].address;
        
        match self.database_conn.update_message_flags(
            username,
            address,
            mailbox_path,
            message_uid,
            flags
        ) {
            Ok(_) => return Ok(flags.to_string()),
            Err(e) => return Err(e),
        };
    }
}

