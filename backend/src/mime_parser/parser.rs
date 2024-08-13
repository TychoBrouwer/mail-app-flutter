use async_imap::types::{Fetch, Flag};
use base64::{prelude::BASE64_STANDARD, Engine};
use regex::Regex;
use std::collections::HashMap;

use crate::mime_parser::decode;
use crate::mime_parser::parse_address;
use crate::mime_parser::parse_time;
use crate::my_error::MyError;
use crate::types::message::Message;

enum MimeParserState {
    HeaderKey,
    HeaderValue,
    TextHeader,
    Text,
    HtmlHeader,
    Html,
    BlankLine,
}

fn parse_message_body(body: &str) -> Message {
    let mut state = MimeParserState::HeaderKey;

    let mut header_key = String::from("");
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut html = String::from("");
    let mut text = String::from("");

    let mut html_encoding = String::from("utf-8");
    let mut text_encoding = String::from("utf-8");

    let lines = body.lines();

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

    let date = parse_time::rfc2822(headers.get("Date"));
    let received = parse_time::rfc2822(headers.get("Received"));

    let binding = String::from("");
    let to = headers.get("To").unwrap_or(&binding);
    let delivered_to = headers.get("Delivered-To").unwrap_or(&binding);
    let from = headers.get("From").unwrap_or(&binding);
    let subject = headers.get("Subject").unwrap_or(&binding);
    let message_id = headers.get("Message-ID").unwrap_or(&binding);

    return Message {
        message_uid: 0,
        sequence_id: 0,
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
        flags: String::from(""),
        text,
        html,
    };
}

pub fn parse_flag_vec(flags: &[Flag]) -> String {
    let mut flags_str = String::from("[");

    for (i, flag) in flags.iter().enumerate() {
        flags_str.push_str(&format!("\"{:?}\"", flag));

        if i < flags.len() - 1 {
            flags_str.push_str(", ");
        }
    }
    flags_str.push_str("]");

    return flags_str;
}

pub fn parse_message_vec(messages: Vec<Message>) -> String {
    let mut result = String::from("[");

    for message in messages {
        result.push_str(&message.to_string());
        result.push_str(",");
    }

    result.pop();
    result.push_str("]");

    return result;
}

pub fn parse_string_vec(strings: Vec<String>) -> String {
    let mut result = String::from("[");

    for string in strings {
        result.push_str(&format!("\"{}\"", string));
        result.push_str(",");
    }

    result.pop();
    result.push_str("]");

    return result;
}

pub fn parse_fetch(fetch: &Fetch) -> Result<Message, MyError> {
    let envelope = match fetch.envelope() {
        Some(e) => e,
        None => {
            let err = MyError::String(
                String::from("Message envelope not found in fetch"),
                String::from("Error parsing message"),
            );
            err.log_error();

            return Err(err);
        }
    };

    let message_uid = match fetch.uid {
        Some(u) => u,
        None => {
            let err = MyError::String(
                String::from("Message UID not found in fetch"),
                String::from("Error parsing message"),
            );
            err.log_error();

            return Err(err);
        }
    };

    let body_str = match fetch.body() {
        Some(b) => match std::str::from_utf8(b) {
            Ok(b) => b,
            Err(e) => {
                let err = MyError::Utf8(e, String::from("Error parsing message"));
                err.log_error();

                return Err(err);
            }
        },
        None => {
            let err = MyError::String(
                String::from("Message body not found in fetch"),
                String::from("Error parsing message"),
            );
            err.log_error();

            return Err(err);
        }
    };

    let flags = fetch.flags().collect::<Vec<Flag>>();
    let flags_str = parse_flag_vec(&flags);

    let body_data = parse_message_body(body_str);

    return Ok(Message {
        message_uid,
        sequence_id: fetch.message,
        message_id: decode::u8(envelope.message_id.as_deref()),
        subject: decode::u8(envelope.subject.as_deref()),
        from: parse_address::to_string(&envelope.from),
        sender: parse_address::to_string(&envelope.sender),
        to: parse_address::to_string(&envelope.to),
        cc: parse_address::to_string(&envelope.cc),
        bcc: parse_address::to_string(&envelope.bcc),
        reply_to: parse_address::to_string(&envelope.reply_to),
        in_reply_to: decode::u8(envelope.in_reply_to.as_deref()),
        delivered_to: body_data.delivered_to,
        date: body_data.date,
        received: body_data.received,
        flags: flags_str,
        text: body_data.text,
        html: body_data.html,
    });
}
