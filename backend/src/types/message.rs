use std::u32;

use base64::{prelude::BASE64_STANDARD, Engine};
use rusqlite::Row;

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
    pub flags: String,
    pub text: String,
    pub html: String,
}

impl Message {
    pub fn from_row(row: &Row) -> Message {
        let html: String = row.get(18).unwrap();
        let text: String = row.get(19).unwrap();

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
            flags: row.get(17).unwrap(),
            html: BASE64_STANDARD.encode(html.as_bytes()),
            text: BASE64_STANDARD.encode(text.as_bytes()),
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::from("{");

        result.push_str(&format!("\"uid\": {},", self.message_uid));
        result.push_str(&format!("\"sequence_id\": {},", self.sequence_id));
        result.push_str(&format!("\"message_id\": \"{}\",", self.message_id));
        result.push_str(&format!("\"subject\": \"{}\",", self.subject));
        result.push_str(&format!("\"from\": {},", self.from));
        result.push_str(&format!("\"sender\": {},", self.sender));
        result.push_str(&format!("\"to\": {},", self.to));
        result.push_str(&format!("\"cc\": {},", self.cc));
        result.push_str(&format!("\"bcc\": {},", self.bcc));
        result.push_str(&format!("\"reply_to\": {},", self.reply_to));
        result.push_str(&format!("\"in_reply_to\": \"{}\",", self.in_reply_to));
        result.push_str(&format!("\"delivered_to\": \"{}\",", self.delivered_to));
        result.push_str(&format!("\"date\": {},", self.date));
        result.push_str(&format!("\"received\": {},", self.received));
        result.push_str(&format!("\"flags\": {},", self.flags));
        result.push_str(&format!("\"html\": \"{}\",", self.html));
        result.push_str(&format!("\"text\": \"{}\"", self.text));

        result.push_str("}");

        return result;
    }
}
