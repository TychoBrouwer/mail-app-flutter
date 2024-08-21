use crate::types::message::Message;

pub fn message_to_display(message: &Message) -> String {
    let result = String::from("{")
        + &format!("\"uid\":{},", message.message_uid)
        + &format!("\"sequence_id\":{},", message.sequence_id)
        + &format!("\"message_id\":\"{}\",", message.message_id)
        + &format!("\"subject\":\"{}\",", message.subject)
        + &format!("\"from\":{},", message.from)
        + &format!("\"sender\":{},", message.sender)
        + &format!("\"to\":{},", message.to)
        + &format!("\"cc\":{},", message.cc)
        + &format!("\"bcc\":{},", message.bcc)
        + &format!("\"reply_to\":{},", message.reply_to)
        + &format!("\"in_reply_to\":\"{}\",", message.in_reply_to)
        + &format!("\"delivered_to\":\"{}\",", message.delivered_to)
        + &format!("\"date\":{},", message.date)
        + &format!("\"received\":{},", message.received)
        + &format!("\"flags\":{},", string_vec_to_display(&message.flags))
        + &format!("\"html\":\"{}\",", message.html)
        + &format!("\"text\":\"{}\"", message.text)
        + "}";

    return result;
}

pub fn message_vec_to_display(messages: &Vec<Message>) -> String {
    let mut result = String::from("[");

    for (i, message) in messages.iter().enumerate() {
        result.push_str(&message_to_display(message));

        if i < messages.len() - 1 {
            result.push_str(",");
        }
    }

    result.push_str("]");

    return result;
}

pub fn string_vec_to_display(strings: &Vec<String>) -> String {
    let mut result = String::from("[");

    for (i, string) in strings.iter().enumerate() {
        result.push_str(&format!("\"{}\"", string));

        if i < strings.len() - 1 {
            result.push_str(",");
        }
    }

    result.push_str("]");

    return result;
}

pub fn u32_vec_to_display(u32s: &Vec<u32>) -> String {
    let mut result = String::from("[");

    for (i, u32) in u32s.iter().enumerate() {
        result.push_str(&u32.to_string());

        if i < u32s.len() - 1 {
            result.push_str(",");
        }
    }

    result.push_str("]");

    return result;
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
    fn message() {
        let message = get_message();

        let expected = r#"{"uid":1,"sequence_id":2,"message_id":"message_id","subject":"subject","from":from,"sender":sender,"to":to,"cc":cc,"bcc":bcc,"reply_to":reply_to,"in_reply_to":"in_reply_to","delivered_to":"delivered_to","date":3,"received":4,"flags":["seen","flagged"],"html":"html","text":"text"}"#;

        assert_eq!(message_to_display(&message), expected);
    }

    #[test]
    fn messages() {
        let messages = vec![get_message(), get_message()];

        let expected = r#"[{"uid":1,"sequence_id":2,"message_id":"message_id","subject":"subject","from":from,"sender":sender,"to":to,"cc":cc,"bcc":bcc,"reply_to":reply_to,"in_reply_to":"in_reply_to","delivered_to":"delivered_to","date":3,"received":4,"flags":["seen","flagged"],"html":"html","text":"text"},{"uid":1,"sequence_id":2,"message_id":"message_id","subject":"subject","from":from,"sender":sender,"to":to,"cc":cc,"bcc":bcc,"reply_to":reply_to,"in_reply_to":"in_reply_to","delivered_to":"delivered_to","date":3,"received":4,"flags":["seen","flagged"],"html":"html","text":"text"}]"#;

        assert_eq!(message_vec_to_display(&messages), expected);
    }

    #[test]
    fn string_vec() {
        let strings = vec![String::from("one"), String::from("two")];

        let expected = r#"["one","two"]"#;

        assert_eq!(string_vec_to_display(&strings), expected);
    }

    #[test]
    fn u32_vec() {
        let u32s = vec![1, 2];

        let expected = r#"[1,2]"#;

        assert_eq!(u32_vec_to_display(&u32s), expected);
    }

    #[test]
    fn empty_string_vec() {
        let strings = vec![];

        let expected = r#"[]"#;

        assert_eq!(string_vec_to_display(&strings), expected);
    }

    #[test]
    fn empty_u32_vec() {
        let u32s = vec![];

        let expected = r#"[]"#;

        assert_eq!(u32_vec_to_display(&u32s), expected);
    }

    #[test]
    fn empty_messages_vec() {
        let messages = vec![];

        let expected = r#"[]"#;

        assert_eq!(message_vec_to_display(&messages), expected);
    }
}
