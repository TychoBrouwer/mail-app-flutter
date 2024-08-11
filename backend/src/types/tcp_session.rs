use async_imap::Session;
use async_native_tls::TlsStream;
use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};

pub type TcpSessions = Arc<Mutex<Vec<Session<TlsStream<TcpStream>>>>>;
