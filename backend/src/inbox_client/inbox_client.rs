use crate::database::conn::DBConnection;

use imap;
use native_tls::{TlsConnector, TlsStream};
use std::net::TcpStream;

#[derive(Debug)]
pub struct SequenceSet {
    pub nr_messages: Option<usize>,
    pub start_end: Option<StartEnd>,
    pub idx: Option<Vec<usize>>,
}

#[derive(Debug)]
pub struct StartEnd {
    pub start: usize,
    pub end: usize,
}

pub struct Session {
    pub stream: Option<imap::Session<TlsStream<TcpStream>>>,
    pub address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

pub struct InboxClient {
    pub database_conn: DBConnection,
    pub sessions: Vec<Session>,
}

impl InboxClient {
    pub fn new(database_conn: DBConnection) -> InboxClient {
        InboxClient {
            database_conn,
            sessions: Vec::new(),
        }
    }

    pub fn connect(
        &mut self,
        session: Session,
    ) -> Result<usize, String> {
        self.sessions.push(session);

        let idx = self.sessions.len() - 1;

        self.database_conn
            .insert_connection(&self.sessions[idx]);

        match self.connect_imap(idx) {
            Ok(_) => {
                return Ok(idx);
            }
            Err(e) => {

                self.sessions.remove(idx);
                return Err(e);
            }
        }
    }

    fn connect_imap(
        &mut self,
        idx: usize,
    ) -> Result<(), String> {
        let tls = TlsConnector::builder().build().unwrap();

        let address = &self.sessions[idx].address;
        let port = self.sessions[idx].port;
        let username = &self.sessions[idx].username;
        let password = &self.sessions[idx].password;

        match imap::connect((address.as_str(), port), address, &tls) {
            Ok(c) => match c.login(username, password) {
                Ok(s) => {
                    self.sessions[idx].stream = Some(s);

                    return Ok(());
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

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => {
                return Err(String::from("Session not found"));
            }
        };

        match session.logout() {
            Ok(_) => {
                self.sessions.remove(session_id);

                return Ok(());
            }
            Err(e) => {
                eprintln!("Error logging out: {:?}", e);
                return Err(String::from("Error logging out"));
            }
        }
    }

    pub fn sequence_set_to_string(sequence_set: &SequenceSet) -> Result<String, String> {
        let sequence_set_string: String = match sequence_set {
            SequenceSet {
                nr_messages: Some(nr_messages),
                start_end: None,
                idx: None,
            } => {
                format!("1:{}", nr_messages)
            }
            SequenceSet {
                nr_messages: None,
                start_end: Some(StartEnd { start, end }),
                idx: None,
            } => {
                if start > end {
                    return Err(String::from("Start must be less than or equal to end"));
                }
    
                format!("{}:{}", start, end)
            }
            SequenceSet {
                nr_messages: None,
                start_end: None,
                idx: Some(idxs),
            } => {                
                let mut result = String::new();
    
                for (i, idx) in idxs.iter().enumerate() {
                    result.push_str(&(idx + 1).to_string());
    
                    if i < idxs.len() - 1 {
                        result.push_str(",");
                    }
                }
    
                result
            }
            _ => String::from("1:*"),
        };
    
        return Ok(sequence_set_string);
    }
}
