use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::{DateTime, FixedOffset};
use imap;
use imap_proto;
use regex::Regex;
use std::collections::HashMap;

pub struct MessageBody {
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
            dbg!("Error: Could not parse date");
            "Thu, 1 Jan 1970 00:00:00 +0000"
        }
    };

    let date = match DateTime::parse_from_rfc2822(&date) {
        Ok(date) => date,
        Err(e) => {
            dbg!("Error: {}", e);
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

fn address_to_string(address: &Option<Vec<imap_proto::types::Address>>) -> String {
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

pub fn parse_message_body(
    body: &str,
    uid: &u32,
) -> MessageBody {
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

                let split = match line.split_once(":") {
                    Some(s) => s,
                    None => {
                        dbg!("Error: Could not split header key and value");

                        ("", "")
                    }
                };

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

                let split = match line.split_once(":") {
                    Some(s) => s,
                    None => {
                        dbg!("Error: Could not split header key and value");

                        ("", "")
                    }
                };

                let key = split.0.trim();

                if key == "Content-Transfer-Encoding" {
                    text_encoding = split.1.trim().to_string();
                }
            }
            MimeParserState::Text => {
                if line.starts_with(&(String::from("--") + boundary)) {
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

                let split = match line.split_once(":") {
                    Some(s) => s,
                    None => {
                        dbg!("Error: Could not split header key and value");

                        ("", "")
                    }
                };

                let key = split.0.trim();

                if key == "Content-Transfer-Encoding" {
                    html_encoding = split.1.trim().to_string();
                }
            }
            MimeParserState::Html => {
                if line.starts_with(&(String::from("--") + boundary)) {
                    state = MimeParserState::BlankLine;

                    continue;
                }

                html.push_str(line);
            }
            MimeParserState::BlankLine => {
                if line.starts_with("Content-Type: text/plain") {
                    state = MimeParserState::TextHeader;
                } else if line.starts_with("Content-Type: text/html") {
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

    return MessageBody {
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

pub fn parse_envelope(
    fetch: &imap::types::Fetch,
    uid: &u32,
) -> Result<MessageBody, String> {
    let envelope = match fetch.envelope() {
        Some(e) => e,
        None => return Err(String::from("Error getting envelope")),
    };

    let date_str = decode_u8(envelope.date);
    let date = parse_time_rfc2822(Some(&date_str)).timestamp_millis();

    return Ok(MessageBody {
        uid: uid.clone(),
        message_id: String::from(""),
        subject: decode_u8(envelope.subject),
        from: address_to_string(&envelope.from),
        sender: String::from(""),
        to: address_to_string(&envelope.to),
        cc: address_to_string(&envelope.cc),
        bcc: address_to_string(&envelope.bcc),
        reply_to: address_to_string(&envelope.reply_to),
        in_reply_to: decode_u8(envelope.in_reply_to),
        delivered_to: String::from(""),
        date,
        received: 0,
        text: String::from(""),
        html: String::from(""),
    });
}

pub fn message_merge(message_1: MessageBody, message_2: MessageBody) -> MessageBody{
    let mut result = message_2;

    if !message_1.message_id.is_empty() { result.message_id = message_1.message_id };
    if !message_1.sender.is_empty() { result.sender = message_1.sender };
    if !message_1.cc.is_empty() { result.cc = message_1.cc };
    if !message_1.bcc.is_empty() { result.bcc = message_1.bcc };
    if !message_1.reply_to.is_empty() { result.reply_to = message_1.reply_to };
    if !message_1.in_reply_to.is_empty() { result.in_reply_to = message_1.in_reply_to };
    if !message_1.delivered_to.is_empty() { result.delivered_to = message_1.delivered_to };
    if message_1.received != 0 { result.received = message_1.received };
    if !message_1.text.is_empty() { result.text = message_1.text };
    if !message_1.html.is_empty() { result.html = message_1.html };

    return result;
}

pub fn message_to_string(message_body: MessageBody) -> String {
    let mut result = String::from("{");

    result.push_str(&format!("\"uid\": {},", message_body.uid));
    result.push_str(&format!("\"message_id\": \"{}\",", message_body.message_id));
    result.push_str(&format!("\"subject\": \"{}\",", message_body.subject));
    result.push_str(&format!("\"from\": \"{}\",", message_body.from));
    result.push_str(&format!("\"sender\": \"{}\",", message_body.sender));
    result.push_str(&format!("\"to\": \"{}\",", message_body.to));
    result.push_str(&format!("\"cc\": \"{}\",", message_body.cc));
    result.push_str(&format!("\"bcc\": \"{}\",", message_body.bcc));
    result.push_str(&format!("\"reply_to\": \"{}\",", message_body.reply_to));
    result.push_str(&format!("\"in_reply_to\": \"{}\",", message_body.in_reply_to));
    result.push_str(&format!("\"delivered_to\": \"{}\",", message_body.delivered_to));
    result.push_str(&format!("\"date\": \"{}\",", message_body.date));
    result.push_str(&format!("\"received\": \"{}\",", message_body.received));
    result.push_str(&format!(
        "\"delivered_to\": \"{}\",",
        message_body.delivered_to
    ));
    result.push_str(&format!("\"html\": \"{}\"", message_body.html));
    result.push_str(&format!("\"text\": \"{}\",", message_body.text));

    result.push_str("}");

    return result;
}
