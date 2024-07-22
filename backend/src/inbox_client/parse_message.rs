use std::collections::HashMap;
use regex::Regex;
use base64::{engine::general_purpose, Engine as _};
use imap;
use imap_proto;

fn message_str(string: Option<&[u8]>) -> String {
    match string {
        Some(s) => match std::str::from_utf8(s) {
            Ok(s) => String::from(s),
            Err(_) => String::from(""),
        },
        None => String::from(""),
    }
}

fn message_address(address: &Option<Vec<imap_proto::types::Address>>) -> String {
    match address {
        Some(a) => {
            let mut result = String::from("[");

            for (i, address) in a.iter().enumerate() {
                result.push_str("{");
                result.push_str(&format!("\"name\": \"{}\",", message_str(address.name)));
                result.push_str(&format!("\"mailbox\": \"{}\",", message_str(address.mailbox)));
                result.push_str(&format!("\"host\": \"{}\"", message_str(address.host)));
                result.push_str("}");

                if i < a.len() - 1 {
                    result.push_str(",");
                }
            }

            result.push_str("]");

            return result;
        },
        None => return String::from("[]"),
    }
}

struct MessageBody {
    headers: HashMap<String, String>,
    text: String,
    html: String,
}

enum State {
    HeaderKey,
    HeaderValue,
    TextHeader,
    Text,
    HtmlHeader,
    Html,
    BlankLine,
}

fn parse_message_body(body: &str) -> MessageBody {   
    let mut state = State::HeaderKey;

    let mut header_key = String::from("");
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut html = String::from("");
    let mut text = String::from("");

    let lines = body.lines();

    let re_boundary = Regex::new(r#"boundary="(.*)""#).unwrap();
    let boundary = match re_boundary.captures(body) {
        Some(c) => c.get(1).unwrap().as_str(),
        None => "",
    };

    dbg!(&boundary);

    let mut i = 0;
    while i < lines.clone().count() {
        let line = lines.clone().nth(i).unwrap();
        i += 1;

        match &state {
            State::HeaderKey => {
                if line.is_empty() {
                    state = State::BlankLine;

                    continue;
                }

                let mut split = line.split(":");
                match split.next() {
                    Some(k) => {
                        header_key = k.to_string();

                        let value_part = split
                            .next()
                            .unwrap_or("")
                            .trim()
                            .replace("\r\n", " ");

                        headers.insert(k.to_string(), value_part);
                    },
                    None => {},
                };

                state = State::HeaderValue;
            },
            State::HeaderValue => {
                if line.is_empty() {
                    state = State::BlankLine;

                    continue;
                } else if line.contains(":") && line.starts_with(char::is_alphabetic) {
                    state = State::HeaderKey;

                    i -= 1;
                    continue;
                }

                let value = line
                    .rsplit(":")
                    .next()
                    .unwrap_or("")
                    .trim()
                    .replace("\r\n", " ");

                if headers.contains_key(&header_key) {
                    headers.insert(header_key.clone(), headers[&header_key].clone() + value.as_str());
                } else {
                    headers.insert(header_key.clone(), value);
                }
            },
            State::TextHeader => {
                if line.is_empty() || (!line.contains(":") && line.starts_with(char::is_alphabetic)) {
                    state = State::Text;

                    continue;
                }
            },
            State::Text => {
                if line.starts_with(&(String::from("--") + boundary)) {
                    state = State::BlankLine;

                    continue;
                }

                text.push_str(line);
            },
            State::HtmlHeader => {
                if line.is_empty() || (!line.contains(":") && line.starts_with(char::is_alphabetic)) {
                    state = State::Html;
                }
            },
            State::Html => {
                if line.starts_with(&(String::from("--") + boundary)) {
                    state = State::BlankLine;

                    continue;
                }

                html.push_str(line);
            },
            State::BlankLine => {

                if line.starts_with("Content-Type: text/plain") {
                    state = State::TextHeader;
                } else if line.starts_with("Content-Type: text/html") {
                    state = State::HtmlHeader;
                }
            }
        }
    }

    let re_encoding = Regex::new(r"=(..)").unwrap();
    html = re_encoding.replace_all(html.as_str(), |caps: &regex::Captures| {
        if caps.get(1).unwrap().as_str() == "3D" {
            String::from("=")
        } else {
            caps.get(1).unwrap().as_str().to_string()
        }
    }).to_string();

    html = html.replace("=3D", "=");
    html = html.replace("&#39;", "'");
    html = html.replace("&amp;", "&");
    html = html.replace("&copy;", "©");

    return MessageBody {
        headers: headers,
        text,
        html,
    };
}

pub fn envelope_to_string(fetch: &imap::types::Fetch, message_uid: u32) -> String {
    let envelope = match fetch.envelope() {
        Some(e) => e,
        None => return String::from(""),
    };

    let mut result = String::from("{");

    result.push_str(&format!("\"date\": \"{}\",", message_str(envelope.date)));
    result.push_str(&format!("\"subject\": \"{}\",", message_str(envelope.subject)));
    result.push_str(&format!("\"from\": {},", message_address(&envelope.from)));
    result.push_str(&format!("\"sender\": {},", message_address(&envelope.sender)));
    result.push_str(&format!("\"reply_to\": {},", message_address(&envelope.reply_to)));
    result.push_str(&format!("\"to\": {},", message_address(&envelope.to)));
    result.push_str(&format!("\"cc\": {},", message_address(&envelope.cc)));
    result.push_str(&format!("\"bcc\": {},", message_address(&envelope.bcc)));
    result.push_str(&format!("\"in_reply_to\": \"{}\",", message_str(envelope.in_reply_to)));
    result.push_str(&format!("\"message_id\": \"{}\",", message_str(envelope.message_id)));
    result.push_str(&format!("\"message_uid\": {}", message_uid));

    result.push_str("}");

    return result;
}

pub fn message_to_string(body_fetch: &imap::types::Fetch, message_uid: u32) -> String {
    let message = match body_fetch.body() {
        Some(m) => std::str::from_utf8(m).unwrap(),
        None => "",
    };

    let message_body = parse_message_body(message);

    let mut result = String::from("{");

    result.push_str(&format!("\"message_uid\": {},", message_uid));
    result.push_str(&format!("\"date\": \"{}\",", ""));
    result.push_str(&format!("\"text\": \"{}\",", message_body.text));
    result.push_str(&format!("\"html\": \"{}\"", general_purpose::STANDARD.encode(&message_body.html)));

    result.push_str("}");

    return result;
}
