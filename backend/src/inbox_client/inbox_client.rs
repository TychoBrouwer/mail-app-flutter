use crate::{database::conn::DBConnection, inbox_client::parse_message};

use imap;
use native_tls::{TlsConnector, TlsStream};
use std::net::TcpStream;

use super::parse_message::MessageBody;

pub struct SequenceSet {
    pub nr_messages: Option<usize>,
    pub start_end: Option<StartEnd>,
}

#[derive(Clone)]
pub struct StartEnd {
    pub start: usize,
    pub end: usize,
}

pub struct InboxClient {
    pub database_conn: DBConnection,
    pub sessions: Vec<imap::Session<TlsStream<TcpStream>>>,
    pub addresses: Vec<String>,
    pub usernames: Vec<String>,
}

impl InboxClient {
    pub fn new(database_conn: DBConnection) -> InboxClient {
        InboxClient {
            database_conn,
            sessions: Vec::new(),
            addresses: Vec::new(),
            usernames: Vec::new(),
        }
    }

    pub fn connect(
        &mut self,
        address: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<usize, String> {
        self.database_conn
            .insert_connection(username, password, address, port);

        return self.connect_imap(address, port, username, password);
    }

    pub fn connect_imap(
        &mut self,
        address: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<usize, String> {
        let tls = TlsConnector::builder().build().unwrap();

        match imap::connect((address, port), address, &tls) {
            Ok(c) => match c.login(username, password) {
                Ok(s) => {
                    self.sessions.push(s);
                    self.addresses.push(String::from(address));
                    self.usernames.push(String::from(username));

                    return Ok(self.sessions.len() - 1);
                }
                Err(e) => {
                    eprintln!("Error logging in: {:?}", e);
                    return Err(String::from("Error logging in"));
                }
            },
            Err(e) => {
                eprintln!("Error connecting to IMAP server: {}", e);
                return Err(String::from("Error connecting to IMAP server"));
            }
        };
    }

    pub fn logout_imap(&mut self, session_id: usize) -> Result<(), String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        match session.logout() {
            Ok(_) => {
                self.sessions.remove(session_id);
                self.addresses.remove(session_id);
                self.usernames.remove(session_id);

                return Ok(());
            }
            Err(e) => {
                eprintln!("Error logging out: {:?}", e);
                return Err(String::from("Error logging out"));
            }
        }
    }

    pub fn get_message_envelopes_imap(
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
            _ => return Err(String::from("Provide either nr_messages or start and end")),
        };

        let message_envelopes = session
            .fetch(sequence_set_string.clone(), "ENVELOPE")
            .unwrap();
        let message_uids = session.fetch(sequence_set_string, "UID").unwrap();

        let mut response = String::from("{\"messages\": [");

        for (i, fetch) in message_envelopes.iter().enumerate() {
            let message_uid = match message_uids[i].uid {
                Some(uid) => uid,
                None => 0,
            };

            let message = match parse_message::parse_envelope(fetch, &message_uid) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error parsing envelope: {:?}", e);
                    return Err(String::from("Error parsing envelope"));
                }
            };

            let message_string = parse_message::message_to_string(message);

            response.push_str(&message_string);
            if i < message_envelopes.len() - 1 {
                response.push_str(",");
            }
        }

        response.push_str("]}");

        Ok(response)
    }

    pub fn get_message(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
    ) -> Result<String, String> {
        let message_db = self.get_message_db(session_id, mailbox_path, message_uid);

        match message_db {
            Ok(message) => {
                return Err(parse_message::message_to_string(message));
            }
            Err(e) => {
                println!("Error getting message from local database: {:?}", e);

                let message_imap =
                    match self.get_message_imap(session_id, mailbox_path, message_uid) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Error getting message from IMAP: {:?}", e);
                            return Err(String::from("{\"message\": \"Error getting message\"}"));
                        }
                    };

                let message_envelope_imap =
                    match self.get_message_envelope_imap(session_id, mailbox_path, message_uid) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Error getting message envelope from IMAP: {:?}", e);
                            return Err(String::from(
                                "{\"message\": \"Error getting message envelope\"}",
                            ));
                        }
                    };

                let message = parse_message::message_merge(message_imap, message_envelope_imap);

                let username = &self.usernames[session_id];
                let address = &self.addresses[session_id];

                match self
                    .database_conn
                    .insert_message(username, address, mailbox_path, &message)
                {
                    Ok(_) => {
                        return Ok(parse_message::message_to_string(message));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    pub fn get_message_envelope_imap(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
    ) -> Result<MessageBody, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        session.select(mailbox_path).unwrap();

        let envelope_fetch = match session.uid_fetch(message_uid.to_string(), "ENVELOPE") {
            Ok(fetch) => fetch,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(String::from("Error fetching message"));
            }
        };

        let message_envelope = match envelope_fetch.first() {
            Some(e) => e,
            None => return Err(String::from("Message not found")),
        };

        match parse_message::parse_envelope(message_envelope, &message_uid) {
            Ok(message_body) => return Ok(message_body),
            Err(e) => {
                eprintln!("Error parsing envelope: {:?}", e);
                return Err(String::from("Error parsing envelope"));
            }
        };
    }

    pub fn get_message_imap(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
    ) -> Result<MessageBody, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        session.select(&mailbox_path).unwrap();

        let messages = match session.uid_fetch(message_uid.to_string(), "BODY[]") {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error fetching message: {:?}", e);
                return Err(String::from("Error fetching message"));
            }
        };

        match messages.first() {
            Some(message) => {
                let message = match message.body() {
                    Some(m) => std::str::from_utf8(m).unwrap(),
                    None => return Err(String::from("Error getting message body")),
                };

                let message_body: MessageBody =
                    parse_message::parse_message_body(message, message_uid);

                return Ok(message_body);
            }
            None => return Err(String::from("Message not found")),
        };
    }

    pub fn get_message_db(
        &mut self,
        session_id: usize,
        mailbox_path: &str,
        message_uid: &u32,
    ) -> Result<MessageBody, String> {
        if session_id >= self.addresses.len() {
            return Err(String::from("Invalid session ID"));
        }

        let username = &self.usernames[session_id];

        let message =
            match self
                .database_conn
                .get_message_with_uid(username, mailbox_path, message_uid)
            {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error getting message: {:?}", e);
                    return Err(String::from("Error getting message"));
                }
            };

        return Ok(message);
    }

    pub fn get_mailboxes(&mut self, session_id: usize) -> Result<String, String> {
        let mailboxes_db = self.get_mailboxes_db(session_id);

        let mailboxes: Vec<String> = match mailboxes_db {
            Ok(mailboxes) => mailboxes,
            Err(e) => {
                eprintln!("Error getting mailboxes from local database: {:?}", e);

                let mailboxes_imap = self.get_mailboxes_imap(session_id);

                match mailboxes_imap {
                    Ok(mailboxes_imap) => mailboxes_imap,
                    Err(e) => {
                        eprintln!("Error getting mailboxes from IMAP: {:?}", e);
                        return Err(String::from("{\"message\": \"Error getting mailboxes\"}"));
                    }
                }
            }
        };

        let mut response = String::from("[");

        for (i, mailbox_path) in mailboxes.iter().enumerate() {
            response.push_str(&mailbox_path);

            let username = &self.usernames[session_id];
            let address = &self.addresses[session_id];

            self.database_conn.insert_mailbox(username, address, mailbox_path);

            if i < mailboxes.len() - 1 {
                response.push_str(",");
            }
        }

        response.push_str("]");

        return Ok(response);
    }

    pub fn get_mailboxes_db(&mut self, session_id: usize) -> Result<Vec<String>, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let username = &self.usernames[session_id];
        let address = &self.addresses[session_id];

        let mailboxes = match self.database_conn.get_mailboxes(username, address) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error getting mailboxes: {:?}", e);
                return Err(String::from("Error getting mailboxes"));
            }
        };

        return Ok(mailboxes);
    }

    pub fn get_mailboxes_imap(&mut self, session_id: usize) -> Result<Vec<String>, String> {
        if session_id >= self.sessions.len() {
            return Err(String::from("Invalid session ID"));
        }

        let session = &mut self.sessions[session_id];

        let mailboxes = session.list(Some(""), Some("*")).unwrap();

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
