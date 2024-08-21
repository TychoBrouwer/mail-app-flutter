use std::vec;

use base64::{prelude::BASE64_STANDARD, Engine};
use rusqlite::Row;

use crate::types::database_request::MessageReturnData;

#[derive(Debug)]
pub struct Message {
    pub message_uid: u32,
    pub sequence_id: u32,
    pub message_id: String,
    pub subject: String,
    pub from: String,
    pub sender: String,
    pub to: String,
    pub cc: String,
    pub bcc: String,
    pub reply_to: String,
    pub in_reply_to: String,
    pub delivered_to: String,
    pub date: i64,
    pub received: i64,
    pub flags: Vec<String>,
    pub text: String,
    pub html: String,
}

impl Message {
    pub fn from_row(row: &Row, return_data: &MessageReturnData) -> Message {
        match return_data {
            MessageReturnData::All => Message::from_row_all(row),
            MessageReturnData::Flags => Message::from_row_flags(row),
            MessageReturnData::AllWithFlags => Message::from_row_all_with_flags(row),
            MessageReturnData::Uid => Message::from_row_uid(row),
        }
    }

    fn from_row_all(row: &Row) -> Message {
        let html: String = row.get(17).unwrap();
        let text: String = row.get(18).unwrap();

        Message {
            message_uid: row.get(0).unwrap(),
            sequence_id: row.get(4).unwrap(),
            message_id: row.get(5).unwrap(),
            subject: row.get(6).unwrap(),
            from: row.get(7).unwrap(),
            sender: row.get(8).unwrap(),
            to: row.get(9).unwrap(),
            cc: row.get(10).unwrap(),
            bcc: row.get(11).unwrap(),
            reply_to: row.get(12).unwrap(),
            in_reply_to: row.get(13).unwrap(),
            delivered_to: row.get(14).unwrap(),
            date: row.get(15).unwrap(),
            received: row.get(16).unwrap(),
            flags: vec![],
            html: BASE64_STANDARD.encode(html.as_bytes()),
            text: BASE64_STANDARD.encode(text.as_bytes()),
        }
    }

    fn from_row_uid(row: &Row) -> Message {
        Message {
            message_uid: row.get(0).unwrap(),
            sequence_id: 0,
            message_id: String::from(""),
            subject: String::from(""),
            from: String::from(""),
            sender: String::from(""),
            to: String::from(""),
            cc: String::from(""),
            bcc: String::from(""),
            reply_to: String::from(""),
            in_reply_to: String::from(""),
            delivered_to: String::from(""),
            date: 0,
            received: 0,
            flags: vec![],
            html: String::from(""),
            text: String::from(""),
        }
    }

    fn from_row_flags(row: &Row) -> Message {
        Message {
            message_uid: row.get(0).unwrap(),
            sequence_id: 0,
            message_id: String::from(""),
            subject: String::from(""),
            from: String::from(""),
            sender: String::from(""),
            to: String::from(""),
            cc: String::from(""),
            bcc: String::from(""),
            reply_to: String::from(""),
            in_reply_to: String::from(""),
            delivered_to: String::from(""),
            date: 0,
            received: 0,
            flags: vec![row.get(1).unwrap()],
            html: String::from(""),
            text: String::from(""),
        }
    }

    fn from_row_all_with_flags(row: &Row) -> Message {
        let html: String = row.get(17).unwrap();
        let text: String = row.get(18).unwrap();

        let flags: Vec<String> = match row.get(20) {
            Ok(flag) => vec![flag],
            Err(_) => vec![],
        };

        Message {
            message_uid: row.get(0).unwrap(),
            sequence_id: row.get(4).unwrap(),
            message_id: row.get(5).unwrap(),
            subject: row.get(6).unwrap(),
            from: row.get(7).unwrap(),
            sender: row.get(8).unwrap(),
            to: row.get(9).unwrap(),
            cc: row.get(10).unwrap(),
            bcc: row.get(11).unwrap(),
            reply_to: row.get(12).unwrap(),
            in_reply_to: row.get(13).unwrap(),
            delivered_to: row.get(14).unwrap(),
            date: row.get(15).unwrap(),
            received: row.get(16).unwrap(),
            flags,
            html: BASE64_STANDARD.encode(html.as_bytes()),
            text: BASE64_STANDARD.encode(text.as_bytes()),
        }
    }
}
