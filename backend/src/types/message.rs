use std::vec;

use base64::{prelude::BASE64_STANDARD, Engine};
use rusqlite::Row;

use crate::{mime_parser::parser, types::database_request::MessageReturnData};

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

    pub fn to_string(&self) -> String {
        let result = String::from("{")
            + &format!("\"uid\":{},", self.message_uid)
            + &format!("\"sequence_id\":{},", self.sequence_id)
            + &format!("\"message_id\":\"{}\",", self.message_id)
            + &format!("\"subject\":\"{}\",", self.subject)
            + &format!("\"from\":{},", self.from)
            + &format!("\"sender\":{},", self.sender)
            + &format!("\"to\":{},", self.to)
            + &format!("\"cc\":{},", self.cc)
            + &format!("\"bcc\":{},", self.bcc)
            + &format!("\"reply_to\":{},", self.reply_to)
            + &format!("\"in_reply_to\":\"{}\",", self.in_reply_to)
            + &format!("\"delivered_to\":\"{}\",", self.delivered_to)
            + &format!("\"date\":{},", self.date)
            + &format!("\"received\":{},", self.received)
            + &format!("\"flags\":{},", parser::parse_string_vec(&self.flags))
            + &format!("\"html\":\"{}\",", self.html)
            + &format!("\"text\":\"{}\"", self.text)
            + "}";

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_message() -> Message {
        Message {
            message_uid: 1,
            sequence_id: 2,
            message_id: String::from("message_id"),
            subject: String::from("subject"),
            from: String::from("from"),
            sender: String::from("sender"),
            to: String::from("to"),
            cc: String::from("cc"),
            bcc: String::from("bcc"),
            reply_to: String::from("reply_to"),
            in_reply_to: String::from("in_reply_to"),
            delivered_to: String::from("delivered_to"),
            date: 3,
            received: 4,
            flags: vec![String::from("seen"), String::from("flagged")],
            text: String::from("text"),
            html: String::from("html"),
        }
    }

    #[test]
    fn to_string() {
        let message = get_message();

        let expected = r#"{"uid":1,"sequence_id":2,"message_id":"message_id","subject":"subject","from":from,"sender":sender,"to":to,"cc":cc,"bcc":bcc,"reply_to":reply_to,"in_reply_to":"in_reply_to","delivered_to":"delivered_to","date":3,"received":4,"flags":["seen","flagged"],"html":"html","text":"text"}"#;

        assert_eq!(message.to_string(), expected);
    }
}
