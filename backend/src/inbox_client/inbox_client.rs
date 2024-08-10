use imap;
use native_tls::TlsConnector;

use crate::database::conn::DBConnection;
use crate::my_error::MyError;
use crate::types::session::Session;

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
            Err(e) => eprintln!("Error inserting connection into database: {:?}", e),
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
                    Err(e) => return Err(e),
                }

                return Ok({});
            }
            imap::Error::Io(_) => {
                eprintln!("Reconnecting to IMAP server");

                match self.connect_imap(session_id) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                return Ok({});
            }
            _ => {}
        }

        return Err(MyError::Imap(e));
    }
}
