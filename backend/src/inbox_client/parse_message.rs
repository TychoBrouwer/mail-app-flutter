use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::{DateTime, FixedOffset};
use imap::types::Fetch;
use imap_proto::types::Address;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    pub uid: u32,
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
    pub text: String,
    pub html: String,
}

enum MimeParserState {
    HeaderKey,
    HeaderValue,
    TextHeader,
    Text,
    HtmlHeader,
    Html,
    BlankLine,
}

fn parse_time_rfc2822(time_str: Option<&String>) -> DateTime<FixedOffset> {
    let time_re =
        Regex::new(r"(\w{1,3}, \d{1,2} \w{1,3} \d{4} \d{2}:\d{2}:\d{2} ([+-]\d{4})?(\w{3})?)")
            .unwrap();
    let binding = String::from("");

    let date = match time_re.captures(time_str.unwrap_or(&binding)) {
        Some(c) => c.get(1).unwrap().as_str(),
        None => {
            eprintln!("Error: Could not parse date");
            "Thu, 1 Jan 1970 00:00:00 +0000"
        }
    };

    let date = match DateTime::parse_from_rfc2822(&date) {
        Ok(date) => date,
        Err(e) => {
            eprintln!("Error: {}", e);
            DateTime::parse_from_rfc2822("Thu, 1 Jan 1970 00:00:00 +0000").unwrap()
        }
    };

    return date;
}

fn decode_u8(string: Option<&[u8]>) -> String {
    match string {
        Some(s) => match std::str::from_utf8(s) {
            Ok(s) => String::from(s),
            Err(_) => String::from(""),
        },
        None => String::from(""),
    }
}

fn address_to_string(address: &Option<Vec<Address>>) -> String {
    match address {
        Some(a) => {
            let mut result = String::from("[");

            for (i, address) in a.iter().enumerate() {
                result.push_str("{");
                result.push_str(&format!("\"name\": \"{}\",", decode_u8(address.name)));
                result.push_str(&format!("\"mailbox\": \"{}\",", decode_u8(address.mailbox)));
                result.push_str(&format!("\"host\": \"{}\"", decode_u8(address.host)));
                result.push_str("}");

                if i < a.len() - 1 {
                    result.push_str(",");
                }
            }

            result.push_str("]");

            return result;
        }
        None => return String::from("[]"),
    }
}

fn parse_message_body(body: &str, uid: &u32) -> Message {
    let mut state = MimeParserState::HeaderKey;

    let mut header_key = String::from("");
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut html = String::from("");
    let mut text = String::from("");

    let mut html_encoding = String::from("utf-8");
    let mut text_encoding = String::from("utf-8");

    let lines = body.lines();

    let re_boundary = Regex::new(r#"boundary="(.*)""#).unwrap();
    let boundary = match re_boundary.captures(body) {
        Some(c) => c.get(1).unwrap().as_str(),
        None => "",
    };

    let mut i = 0;
    while i < lines.clone().count() {
        let line = lines.clone().nth(i).unwrap();
        i += 1;

        match &state {
            MimeParserState::HeaderKey => {
                if line.is_empty() {
                    state = MimeParserState::BlankLine;

                    continue;
                }

                let split = line.split_once(":").unwrap_or(("", ""));

                header_key = split.0.to_string();

                let value_part = split.1.trim().replace("\r\n", " ");

                if headers.contains_key(&header_key) {
                    headers.insert(
                        header_key.clone(),
                        headers[&header_key].clone() + value_part.as_str(),
                    );
                } else {
                    headers.insert(header_key.clone(), value_part);
                }

                state = MimeParserState::HeaderValue;
            }
            MimeParserState::HeaderValue => {
                if line.is_empty() {
                    state = MimeParserState::BlankLine;

                    continue;
                } else if line.contains(":") && line.starts_with(char::is_alphabetic) {
                    state = MimeParserState::HeaderKey;

                    i -= 1;
                    continue;
                }

                let value = line.trim().replace("\r\n", " ");

                if headers.contains_key(&header_key) {
                    headers.insert(
                        header_key.clone(),
                        headers[&header_key].clone() + value.as_str(),
                    );
                } else {
                    headers.insert(header_key.clone(), value);
                }
            }
            MimeParserState::TextHeader => {
                if line.is_empty() || (!line.contains(":") && line.starts_with(char::is_alphabetic))
                {
                    state = MimeParserState::Text;

                    continue;
                }

                let split = line.split_once(":").unwrap_or(("", ""));

                let key = split.0.trim();

                if key == "Content-Transfer-Encoding" {
                    text_encoding = split.1.trim().to_string();
                }
            }
            MimeParserState::Text => {
                if line.starts_with(&(String::from("--"))) {
                    state = MimeParserState::BlankLine;

                    continue;
                }

                text.push_str(line);
            }
            MimeParserState::HtmlHeader => {
                if line.is_empty() || (!line.contains(":") && line.starts_with(char::is_alphabetic))
                {
                    state = MimeParserState::Html;

                    continue;
                }

                let split = line.split_once(":").unwrap_or(("", ""));

                let key = split.0.trim();

                if key == "Content-Transfer-Encoding" {
                    html_encoding = split.1.trim().to_string();
                }
            }
            MimeParserState::Html => {
                if line.starts_with(&(String::from("--"))) {
                    state = MimeParserState::BlankLine;

                    continue;
                }

                html.push_str(line);
            }
            MimeParserState::BlankLine => {
                if line.starts_with("Content-Type: text/plain") && text.is_empty() {
                    state = MimeParserState::TextHeader;
                } else if line.starts_with("Content-Type: text/html") && html.is_empty() {
                    state = MimeParserState::HtmlHeader;
                }
            }
        }
    }

    let re_encoding = Regex::new(r"=(..)").unwrap();
    html = re_encoding
        .replace_all(html.as_str(), |caps: &regex::Captures| {
            if caps.get(1).unwrap().as_str() == "3D" {
                String::from("=")
            } else {
                caps.get(1).unwrap().as_str().to_string()
            }
        })
        .to_string();

    html = html.replace("=3D", "=");
    html = html.replace("&#39;", "'");
    html = html.replace("&amp;", "&");
    html = html.replace("&copy;", "©");
    html = html.replace("E28099", "'");
    html = html.replace("C2A0", " ");

    if text_encoding != "base64" {
        text = BASE64_STANDARD.encode(text.as_bytes());
    }

    if html_encoding != "base64" {
        html = BASE64_STANDARD.encode(html.as_bytes());
    }

    let date = parse_time_rfc2822(headers.get("Date"));
    let received = parse_time_rfc2822(headers.get("Received"));

    let binding = String::from("");
    let to = headers.get("To").unwrap_or(&binding);
    let delivered_to = headers.get("Delivered-To").unwrap_or(&binding);
    let from = headers.get("From").unwrap_or(&binding);
    let subject = headers.get("Subject").unwrap_or(&binding);
    let message_id = headers.get("Message-ID").unwrap_or(&binding);

    return Message {
        uid: uid.clone(),
        message_id: message_id.to_string(),
        subject: subject.to_string(),
        from: from.to_string(),
        sender: String::from(""),
        to: to.to_string(),
        cc: String::from(""),
        bcc: String::from(""),
        reply_to: String::from(""),
        in_reply_to: String::from(""),
        delivered_to: delivered_to.to_string(),
        date: date.timestamp_millis(),
        received: received.timestamp_millis(),
        text,
        html,
    };
}

pub fn parse_message(fetch: &Fetch) -> Result<Message, String> {
    let envelope = match fetch.envelope() {
        Some(e) => e,
        None => return Err(String::from("Error getting envelope")),
    };

    let uid = match fetch.uid {
        Some(u) => u,
        None => return Err(String::from("Error getting UID")),
    };

    let body_str = match fetch.body() {
        Some(b) => match std::str::from_utf8(b) {
            Ok(b) => b,
            Err(_) => return Err(String::from("Error getting body")),
        },
        None => return Err(String::from("Error getting body")),
    };

    let body_data = parse_message_body(body_str, &uid);

    return Ok(Message {
        uid,
        message_id: decode_u8(envelope.message_id),
        subject: decode_u8(envelope.subject),
        from: address_to_string(&envelope.from),
        sender: address_to_string(&envelope.sender),
        to: address_to_string(&envelope.to),
        cc: address_to_string(&envelope.cc),
        bcc: address_to_string(&envelope.bcc),
        reply_to: address_to_string(&envelope.reply_to),
        in_reply_to: decode_u8(envelope.in_reply_to),
        delivered_to: body_data.delivered_to,
        date: body_data.date,
        received: body_data.received,
        text: body_data.text,
        html: body_data.html,
    });
}

pub fn message_to_string(message: &Message) -> String {
    let mut result = String::from("{");

    result.push_str(&format!("\"uid\": {},", message.uid));
    result.push_str(&format!("\"message_id\": \"{}\",", message.message_id));
    result.push_str(&format!("\"subject\": \"{}\",", message.subject));
    result.push_str(&format!("\"from\": {},", message.from));
    result.push_str(&format!("\"sender\": {},", message.sender));
    result.push_str(&format!("\"to\": {},", message.to));
    result.push_str(&format!("\"cc\": {},", message.cc));
    result.push_str(&format!("\"bcc\": {},", message.bcc));
    result.push_str(&format!("\"reply_to\": {},", message.reply_to));
    result.push_str(&format!("\"in_reply_to\": \"{}\",", message.in_reply_to));
    result.push_str(&format!("\"delivered_to\": \"{}\",", message.delivered_to));
    result.push_str(&format!("\"date\": {},", message.date));
    result.push_str(&format!("\"received\": {},", message.received));
    result.push_str(&format!("\"html\": \"{}\",", message.html));
    result.push_str(&format!("\"text\": \"{}\"", message.text));

    result.push_str("}");

    return result;
}
