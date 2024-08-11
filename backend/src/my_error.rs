use async_imap::error::Error as ImapError;
use async_native_tls::Error as TlsError;
use base64::DecodeError;
use rusqlite::Error as SqliteError;
use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::num::ParseIntError;
use std::str::{ParseBoolError, Utf8Error};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum MyError {
    String(String),
    Imap(ImapError),
    Sqlite(SqliteError),
    FromUtf8(FromUtf8Error),
    Utf8(Utf8Error),
    Base64(DecodeError),
    ParseInt(ParseIntError),
    ParseBool(ParseBoolError),
    Tls(TlsError),
    Io(IoError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::String(str) => write!(f, "GENERAL: {}", str),
            MyError::Imap(err) => write!(f, "IMAP: {}", err),
            MyError::Sqlite(err) => write!(f, "SQLITE: {}", err),
            MyError::FromUtf8(err) => write!(f, "FROM_UTF8: {}", err),
            MyError::Utf8(err) => write!(f, "UTF8: {}", err),
            MyError::Base64(err) => write!(f, "BASE64: {}", err),
            MyError::ParseInt(err) => write!(f, "PARSE_INT: {}", err),
            MyError::ParseBool(str) => write!(f, "PARSE_BOOL: {}", str),
            MyError::Tls(err) => write!(f, "TLS: {}", err),
            MyError::Io(err) => write!(f, "IO: {}", err),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            MyError::String(_) => None,
            MyError::Imap(ref e) => Some(e),
            MyError::Sqlite(ref e) => Some(e),
            MyError::FromUtf8(ref e) => Some(e),
            MyError::Utf8(ref e) => Some(e),
            MyError::Base64(ref e) => Some(e),
            MyError::ParseInt(ref e) => Some(e),
            MyError::ParseBool(_) => None,
            MyError::Tls(ref e) => Some(e),
            MyError::Io(ref e) => Some(e),
        }
    }
}

impl From<ImapError> for MyError {
    fn from(err: ImapError) -> MyError {
        MyError::Imap(err)
    }
}

impl From<SqliteError> for MyError {
    fn from(err: SqliteError) -> MyError {
        MyError::Sqlite(err)
    }
}

impl From<FromUtf8Error> for MyError {
    fn from(err: FromUtf8Error) -> MyError {
        MyError::FromUtf8(err)
    }
}

impl From<Utf8Error> for MyError {
    fn from(err: Utf8Error) -> MyError {
        MyError::Utf8(err)
    }
}

impl From<DecodeError> for MyError {
    fn from(err: DecodeError) -> MyError {
        MyError::Base64(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::ParseInt(err)
    }
}

impl From<ParseBoolError> for MyError {
    fn from(err: ParseBoolError) -> MyError {
        MyError::ParseBool(err)
    }
}

impl From<TlsError> for MyError {
    fn from(err: TlsError) -> MyError {
        MyError::Tls(err)
    }
}

impl From<IoError> for MyError {
    fn from(err: IoError) -> MyError {
        MyError::Io(err)
    }
}
