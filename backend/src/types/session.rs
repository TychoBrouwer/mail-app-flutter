use async_native_tls::TlsStream;
use async_std::net::TcpStream;

pub struct Session {
    pub stream: Option<async_imap::Session<TlsStream<TcpStream>>>,
    pub address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}
