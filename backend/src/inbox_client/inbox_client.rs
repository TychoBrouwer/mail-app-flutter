use crate::database::conn::DBConnection;
use crate::my_error::MyError;

use imap;
use native_tls::{TlsConnector, TlsStream};
use std::net::TcpStream;

#[derive(Debug)]
pub struct SequenceSet {
    pub nr_messages: Option<u32>,
    pub start_end: Option<StartEnd>,
    pub idx: Option<Vec<u32>>,
}

#[derive(Debug)]
pub struct StartEnd {
    pub start: u32,
    pub end: u32,
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

    pub fn connect(&mut self, session: Session) -> Result<usize, MyError> {
        if (self
            .sessions
            .iter()
            .position(|x| x.username == session.username && x.address == session.address))
        .is_some()
        {
            return Err(MyError::String(
                "Already connected to IMAP server".to_string(),
            ));
        }

        self.sessions.push(session);

        let idx = self.sessions.len() - 1;

        match self.database_conn.insert_connection(&self.sessions[idx]) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error inserting connection into database: {:?}", e);
            }
        }

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

    pub fn connect_imap(&mut self, session_id: usize) -> Result<(), MyError> {
        let tls = TlsConnector::builder().build().unwrap();

        if session_id >= self.sessions.len() {
            return Err(MyError::String("Session not found".to_string()));
        }

        let address = &self.sessions[session_id].address;
        let port = self.sessions[session_id].port;
        let username = &self.sessions[session_id].username;
        let password = &self.sessions[session_id].password;

        match imap::connect((address.as_str(), port), address, &tls) {
            Ok(c) => match c.login(username, password) {
                Ok(s) => {
                    self.sessions[session_id].stream = Some(s);

                    match self.get_mailboxes(session_id) {
                        Ok(_) => {
                            return Ok(());
                        }
                        Err(e) => {
                            eprintln!("Error getting mailboxes: {:?}", e);
                            return Err(MyError::String(e.to_string()));
                        }
                    }

                    // return Ok(());
                }
                Err(e) => {
                    eprintln!("Error logging in: {:?}", e);
                    return Err(MyError::Imap(e.0));
                }
            },
            Err(e) => {
                eprintln!("Error connecting to IMAP server: {}", e);
                return Err(MyError::Imap(e));
            }
        };
    }

    pub fn logout_imap(&mut self, session_id: usize) -> Result<(), MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Session not found".to_string()));
        }

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => {
                return Err(MyError::String("Session not found".to_string()));
            }
        };

        match session.logout() {
            Ok(_) => {
                self.sessions.remove(session_id);

                return Ok(());
            }
            Err(e) => {
                eprintln!("Error logging out: {:?}", e);
                return Err(MyError::Imap(e));
            }
        }
    }

    pub fn handle_disconnect(&mut self, session_id: usize, e: imap::Error) -> Result<(), MyError> {
        eprintln!("IMAP communication error: {:?}", e);

        match e {
            imap::Error::ConnectionLost => {
                eprintln!("Reconnecting to IMAP server");

                match self.connect_imap(session_id) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                return Ok({});
            }
            imap::Error::Io(_) => {
                eprintln!("Reconnecting to IMAP server");

                match self.connect_imap(session_id) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                return Ok({});
            }
            _ => {}
        }

        return Err(MyError::Imap(e));
    }

    pub fn sequence_set_to_string(
        sequence_set: &SequenceSet,
        exists: u32,
        reversed: bool,
    ) -> Result<String, MyError> {
        let sequence_set_string: String = match sequence_set {
            SequenceSet {
                nr_messages: Some(nr_messages),
                start_end: None,
                idx: None,
            } => {
                if reversed {
                    let begin = exists - nr_messages + 1;
                    format!("{}:{}", begin, exists)
                } else {
                    format!("1:{}", nr_messages)
                }
            }
            SequenceSet {
                nr_messages: None,
                start_end: Some(StartEnd { start, end }),
                idx: None,
            } => {
                if start > end {
                    return Err(MyError::String(
                        "Start must be less than or equal to end".to_string(),
                    ));
                }

                if reversed {
                    let mut begin = exists - end + 1;
                    let mut last = exists - start + 1;

                    if exists < end + 1 {
                        begin = 1;
                    }

                    if exists < start + 1 {
                        last = 1;
                    }

                    if exists < end + 1 && exists < start + 1 {
                        begin = u32::MAX;
                        last = u32::MAX;
                    }

                    format!("{}:{}", begin, last)
                } else {
                    format!("{}:{}", start, end)
                }
            }
            SequenceSet {
                nr_messages: None,
                start_end: None,
                idx: Some(idxs),
            } => {
                let mut result = String::new();

                for (i, idx) in idxs.iter().enumerate() {
                    if reversed {
                        result.push_str(&((exists - idx + 1).to_string()));
                    } else {
                        result.push_str(&idx.to_string());
                    }

                    if i < idxs.len() - 1 {
                        result.push_str(",");
                    }
                }

                result
            }
            _ => {
                if reversed {
                    format!("{}:*", exists)
                } else {
                    String::from("1:*")
                }
            }
        };

        return Ok(sequence_set_string);
    }
}
