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
    String(String, String),
    Imap(ImapError, String),
    Sqlite(SqliteError, String),
    FromUtf8(FromUtf8Error, String),
    Utf8(Utf8Error, String),
    Base64(DecodeError, String),
    ParseInt(ParseIntError, String),
    ParseBool(ParseBoolError, String),
    Tls(TlsError, String),
    Io(IoError, String),
}

impl MyError {
    pub fn log_error(&self) {
        eprintln!("{}", format!("{}", self));
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::String(err, context) => write!(f, "Error - {}: {}", context, err),
            MyError::Imap(err, context) => write!(f, "Error - {}: {}", context, err),
            MyError::Sqlite(err, context) => write!(f, "Error - {}: {}", context, err),
            MyError::FromUtf8(err, context) => write!(f, "Error - {}: {}", context, err),
            MyError::Utf8(err, context) => write!(f, "Error - {}: {}", context, err),
            MyError::Base64(err, context) => write!(f, "Error - {}: {}", context, err),
            MyError::ParseInt(err, context) => write!(f, "Error - {}: {}", context, err),
            MyError::ParseBool(err, context) => write!(f, "Error - {}: {}", context, err),
            MyError::Tls(err, context) => write!(f, "Error - {}: {}", context, err),
            MyError::Io(err, context) => write!(f, "Error - {}: {}", context, err),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::String(_, _) => None,
            MyError::Imap(ref e, _) => Some(e),
            MyError::Sqlite(ref e, _) => Some(e),
            MyError::FromUtf8(ref e, _) => Some(e),
            MyError::Utf8(ref e, _) => Some(e),
            MyError::Base64(ref e, _) => Some(e),
            MyError::ParseInt(ref e, _) => Some(e),
            MyError::ParseBool(_, _) => None,
            MyError::Tls(ref e, _) => Some(e),
            MyError::Io(ref e, _) => Some(e),
        }
    }
}

impl From<(ImapError, String)> for MyError {
    fn from(err_context: (ImapError, String)) -> MyError {
        let err = MyError::Imap(err_context.0, err_context.1);
        return err;
    }
}

impl From<(SqliteError, String)> for MyError {
    fn from(err_context: (SqliteError, String)) -> MyError {
        let err = MyError::Sqlite(err_context.0, err_context.1);
        return err;
    }
}

impl From<(FromUtf8Error, String)> for MyError {
    fn from(err_context: (FromUtf8Error, String)) -> MyError {
        let err = MyError::FromUtf8(err_context.0, err_context.1);
        return err;
    }
}

impl From<(Utf8Error, String)> for MyError {
    fn from(err_context: (Utf8Error, String)) -> MyError {
        let err = MyError::Utf8(err_context.0, err_context.1);
        return err;
    }
}

impl From<(DecodeError, String)> for MyError {
    fn from(err_context: (DecodeError, String)) -> MyError {
        let err = MyError::Base64(err_context.0, err_context.1);
        return err;
    }
}

impl From<(ParseIntError, String)> for MyError {
    fn from(err_context: (ParseIntError, String)) -> MyError {
        let err = MyError::ParseInt(err_context.0, err_context.1);
        return err;
    }
}

impl From<(ParseBoolError, String)> for MyError {
    fn from(err_context: (ParseBoolError, String)) -> MyError {
        let err = MyError::ParseBool(err_context.0, err_context.1);
        return err;
    }
}

impl From<(TlsError, String)> for MyError {
    fn from(err_context: (TlsError, String)) -> MyError {
        let err = MyError::Tls(err_context.0, err_context.1);
        return err;
    }
}

impl From<(IoError, String)> for MyError {
    fn from(err_context: (IoError, String)) -> MyError {
        let err = MyError::Io(err_context.0, err_context.1);
        return err;
    }
}
