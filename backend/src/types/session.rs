use native_tls::TlsStream;
use std::net::TcpStream;

pub struct Session {
    pub stream: Option<imap::Session<TlsStream<TcpStream>>>,
    pub address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}
