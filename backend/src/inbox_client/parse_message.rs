use std::borrow::Cow;

use imap;
use imap_proto;
use mail_parser::{self, Message};
// use regex::Regex;

fn message_u8(nr: Option<&[u8]>) -> String {
    match nr {
        Some(nr) => match std::str::from_utf8(nr) {
            Ok(nr) => String::from(nr),
            Err(_) => String::from(""),
        },
        None => String::from(""),
    }
}

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

pub fn envelope_to_string(fetch: &imap::types::Fetch, message_uid: u32) -> String {
    let envelope = match fetch.envelope() {
        Some(e) => e,
        None => return String::from(""),
    };

    let mut result = String::from("{");

    result.push_str(&format!("\"date\": \"{}\",", message_u8(envelope.date)));
    result.push_str(&format!("\"subject\": \"{}\",", message_str(envelope.subject)));
    result.push_str(&format!("\"from\": {},", message_address(&envelope.from)));
    result.push_str(&format!("\"sender\": {},", message_address(&envelope.sender)));
    result.push_str(&format!("\"reply_to\": {},", message_address(&envelope.reply_to)));
    result.push_str(&format!("\"to\": {},", message_address(&envelope.to)));
    result.push_str(&format!("\"cc\": {},", message_address(&envelope.cc)));
    result.push_str(&format!("\"bcc\": {},", message_address(&envelope.bcc)));
    result.push_str(&format!("\"in_reply_to\": \"{}\",", message_u8(envelope.in_reply_to)));
    result.push_str(&format!("\"message_id\": \"{}\",", message_u8(envelope.message_id)));
    result.push_str(&format!("\"message_uid\": {}", message_uid));

    result.push_str("}");

    return result;
}

pub fn message_to_string(fetch: &imap::types::Fetch, message_uid: u32) -> String {
    let message = match fetch.text() {
        Some(m) => std::str::from_utf8(m).unwrap(),
        None => "",
    };

    let message = mail_parser::MessageParser::default().parse(message).unwrap();

    dbg!(&message);
    dbg!(&message.from());
    dbg!(&message.html_body);

    let body_html: String = match message.body_html(0) {
        Some(b) => b.into_owned(),
        None => String::from(""),
    };

    let body_text = match message.body_text(0) {
        Some(b) => b,
        None => Cow::Borrowed(""),
    };

    let date = match message.date() {
        Some(d) => d.to_rfc3339(),
        None => String::from(""),
    };


    // let message_parts = message.split("Content-Type: text");

    // let amp_html = message_parts.clone().into_iter().find(|part| part.contains("/x-amp-html")).unwrap();
    // let html = message_parts.clone().into_iter().find(|part| part.contains("/html")).unwrap();
    // let plain = message_parts.clone().into_iter().find(|part| part.contains("/plain")).unwrap();


    // let re: Regex = Regex::new(r"(<.*html.*>.*<\/html>)").unwrap();
    // let html = html.replace("=\r\n", "").replace("\r\n", "");

    // dbg!(&html);

    // let html = re.find(html.as_str()).unwrap().as_str();

    // dbg!(html);
    // dbg!(&html);
    // dbg!(&plain);

    let mut result = String::from("{");

    result.push_str(&format!("\"message_uid\": {},", message_uid));
    result.push_str(&format!("\"date\": \"{}\",", date));
    result.push_str(&format!("\"text\": \"{}\",", body_text.to_string()));
    result.push_str(&format!("\"html\": \"{}\"", body_html.to_string()));


    result.push_str("}");

    return result;
}
