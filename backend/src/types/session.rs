use async_native_tls::TlsStream;
use async_std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Client {
    pub address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

pub type Session = async_imap::Session<TlsStream<TcpStream>>;
