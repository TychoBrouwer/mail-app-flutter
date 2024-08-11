use async_imap;
use async_native_tls::TlsConnector;
use async_std::net::TcpStream;

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

    pub async fn connect(&mut self, session: Session) -> Result<usize, MyError> {
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

        match self.connect_imap(idx).await {
            Ok(_) => {
                return Ok(idx);
            }
            Err(e) => {
                self.sessions.remove(idx);
                return Err(e);
            }
        }
    }

    fn tls() -> TlsConnector {
        TlsConnector::new()
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true)
    }

    pub async fn connect_imap(&mut self, session_id: usize) -> Result<(), MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Session not found".to_string()));
        }

        let address = &self.sessions[session_id].address;
        let port = self.sessions[session_id].port;
        let username = &self.sessions[session_id].username;
        let password = &self.sessions[session_id].password;

        let tcp_stream = match TcpStream::connect((address.as_str(), port)).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error connecting to IMAP server: {}", e);
                return Err(MyError::Io(e));
            }
        };

        let tls = Self::tls();
        let tls_stream = match tls.connect(address, tcp_stream).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error establishing TLS connection: {}", e);
                return Err(MyError::Tls(e));
            }
        };

        let mut client = async_imap::Client::new(tls_stream);
        let _greeting = match client.read_response().await {
            Some(Ok(g)) => g,
            Some(Err(e)) => {
                eprintln!("Error reading greeting: {:?}", e);
                return Err(MyError::Io(e));
            }
            None => {
                return Err(MyError::String("No greeting received".to_string()));
            }
        };

        let session = match client
            .login(username, password)
            .await
            .map_err(|(err, _client)| err)
        {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error logging in: {:?}", e);
                return Err(MyError::Imap(e));
            }
        };

        self.sessions[session_id].stream = Some(session);

        return Ok({});
    }

    pub async fn logout_imap(&mut self, session_id: usize) -> Result<(), MyError> {
        if session_id >= self.sessions.len() {
            return Err(MyError::String("Session not found".to_string()));
        }

        let session = match &mut self.sessions[session_id].stream {
            Some(s) => s,
            None => {
                return Err(MyError::String("Session not found".to_string()));
            }
        };

        match session.logout().await {
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

    pub async fn handle_disconnect(
        &mut self,
        session_id: usize,
        e: async_imap::error::Error,
    ) -> Result<(), MyError> {
        eprintln!("IMAP communication error: {:?}", e);

        match e {
            async_imap::error::Error::ConnectionLost => {
                eprintln!("Reconnecting to IMAP server");

                match self.connect_imap(session_id).await {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                return Ok({});
            }
            async_imap::error::Error::Io(_) => {
                eprintln!("Reconnecting to IMAP server");

                match self.connect_imap(session_id).await {
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
