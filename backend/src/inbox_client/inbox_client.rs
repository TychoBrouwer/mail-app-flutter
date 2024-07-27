use crate::database::conn::DBConnection;

use imap;
use native_tls::{TlsConnector, TlsStream};
use std::net::TcpStream;

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

    // pub fn get_message_envelopes_imap(
    //     &mut self,
    //     session_id: usize,
    //     mailbox_path: &str,
    //     sequence_set: SequenceSet,
    // ) -> Result<String, String> {
    //     if session_id >= self.sessions.len() {
    //         return Err(String::from("Invalid session ID"));
    //     }

    //     let session = &mut self.sessions[session_id];

    //     session.select(mailbox_path).unwrap();

    //     let sequence_set_string: String = match sequence_set {
    //         SequenceSet {
    //             nr_messages: Some(nr_messages),
    //             start_end: None,
    //         } => {
    //             format!("1:{}", nr_messages)
    //         }
    //         SequenceSet {
    //             nr_messages: None,
    //             start_end: Some(StartEnd { start, end }),
    //         } => {
    //             if start > end {
    //                 return Err(String::from("Start must be less than or equal to end"));
    //             }

    //             format!("{}:{}", start, end)
    //         }
    //         _ => return Err(String::from("Provide either nr_messages or start and end")),
    //     };

    //     let message_envelopes = session
    //         .fetch(sequence_set_string.clone(), "ENVELOPE")
    //         .unwrap();
    //     let message_uids = session.fetch(sequence_set_string, "UID").unwrap();

    //     let mut response = String::from("{\"messages\": [");

    //     for (i, fetch) in message_envelopes.iter().enumerate() {
    //         let message_uid = match message_uids[i].uid {
    //             Some(uid) => uid,
    //             None => 0,
    //         };

    //         let message = match parse_message::parse_envelope(fetch, &message_uid) {
    //             Ok(m) => m,
    //             Err(e) => {
    //                 eprintln!("Error parsing envelope: {:?}", e);
    //                 return Err(String::from("Error parsing envelope"));
    //             }
    //         };

    //         let message_string = parse_message::message_to_string(message);

    //         response.push_str(&message_string);
    //         if i < message_envelopes.len() - 1 {
    //             response.push_str(",");
    //         }
    //     }

    //     response.push_str("]}");

    //     Ok(response)
    // }
}
