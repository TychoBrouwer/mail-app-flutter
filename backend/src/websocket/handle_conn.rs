use crate::{
    inbox_client::inbox_client::{InboxClient, SequenceSet, Session, StartEnd},
    websocket::params,
};

pub fn login(uri: &str, inbox_client: &mut InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let email = uri_params.get("email");
    let password = uri_params.get("password");
    let address = uri_params.get("address");
    let port = params::get_u16(uri_params.get("port"));

    if email.is_none() || password.is_none() || address.is_none() || port.is_none() {
        eprintln!("Provide all GET parameters: {}", uri);
        return String::from(
            "{\"success\": \"false\", \"message\": \"Provide all GET parameters\"}",
        );
    }

    let email = email.unwrap();
    let password = password.unwrap();
    let address = address.unwrap();
    let port = port.unwrap();

    match inbox_client
        .sessions
        .iter()
        .position(|x| x.username == email.to_string())
    {
        Some(idx) => {
            return format!("{{\"success\": \"true\", \"message\": \"Allready connected to IMAP server\", \"data\": {{ \"id\": {}}}}}", idx);
        }
        None => {}
    };

    match inbox_client.connect(Session {
        stream: None,
        username: email.to_string(),
        password: password.to_string(),
        address: address.to_string(),
        port,
    }) {
        Ok(idx) => {
            format!("{{\"success\": \"true\", \"message\": \"Connected to IMAP server\", \"data\": {{ \"id\": {}}}}}", idx)
        }
        Err(e) => {
            eprintln!("Error connecting to IMAP server: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": \"{}\"}}", e);
        }
    }
}

pub fn logout(uri: &str, inbox_client: &mut InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = params::get_usize(uri_params.get("session_id"));

    if session_id.is_none() {
        eprintln!("Provide session_id GET parameter: {}", uri);
        return String::from(
            "{\"success\": \"false\", \"message\": \"Provide session_id GET parameter\"}",
        );
    }

    let session_id = session_id.unwrap();

    match inbox_client.logout_imap(session_id) {
        Ok(_) => {
            return String::from("{\"success\": \"true\", \"message\": \"Logged out\"}");
        }
        Err(e) => {
            eprintln!("Error logging out: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": \"{}\"}}", e);
        }
    }
}

pub fn sessions(inbox_client: &mut InboxClient) -> String {
    let sessions = &inbox_client.sessions;

    let mut response = String::from("[");
    for (i, session) in sessions.iter().enumerate() {
        response.push_str(&format!(
            "{{\"id\": {}, \"username\": \"{}\", \"address\": \"{}\", \"port\": {}}}",
            i, session.username, session.address, session.port
        ));

        if i < sessions.len() - 1 {
            response.push_str(",");
        }
    }
    response.push_str("]");

    return response;
}

pub fn mailboxes(uri: &str, inbox_client: &mut InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = params::get_usize(uri_params.get("session_id"));

    if session_id.is_none() {
        eprintln!("Provide session_id GET parameter: {}", uri);
        return String::from(
            "{\"success\": \"false\", \"message\": \"Provide session_id GET parameter\"}",
        );
    }

    let session_id = session_id.unwrap();

    match inbox_client.get_mailboxes(session_id) {
        Ok(mailboxes) => {
            return format!(
                "{{\"success\": \"true\", \"message\": \"Mailboxes retrieved\", \"data\": {}}}",
                mailboxes
            )
        }
        Err(e) => {
            eprintln!("Error getting mailboxes: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": \"{}\"}}", e);
        }
    }
}

pub fn message(uri: &str, inbox_client: &mut InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = params::get_usize(uri_params.get("session_id"));
    let mailbox = uri_params.get("mailbox");
    let message_uid = params::get_u32(uri_params.get("message_uid"));

    if session_id.is_none() || mailbox.is_none() || message_uid.is_none() {
        eprintln!(
            "Provide session_id, mailbox and message_id GET parameters: {}",
            uri
        );
        return String::from("{\"success\": \"false\", \"message\": \"Provide session_id, mailbox and message_uid GET parameters\"}");
    }

    let session_id = session_id.unwrap();
    let mailbox = mailbox.unwrap();
    let message_uid = message_uid.unwrap();

    match inbox_client.get_message(session_id, mailbox, &message_uid) {
        Ok(message) => {
            return format!(
                "{{\"success\": \"true\", \"message\": \"Message retrieved\", \"data\": {}}}",
                message
            )
        }
        Err(e) => {
            eprintln!("Error getting message: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": {}}}", e);
        }
    }
}

pub fn messages(uri: &str, inbox_client: &mut InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = params::get_usize(uri_params.get("session_id"));
    let mailbox = uri_params.get("mailbox");

    let nr_messages = params::get_usize(uri_params.get("nr_messages"));
    let start = params::get_usize(uri_params.get("start"));
    let end = params::get_usize(uri_params.get("end"));

    if session_id.is_none() || mailbox.is_none() {
        eprintln!("Provide session_id GET parameter: {}", uri);
        return String::from(
            "{\"success\": \"false\", \"message\": \"Provide session_id GET parameter\"}",
        );
    }

    let session_id = session_id.unwrap();
    let mailbox = mailbox.unwrap();
    let sequence_set = SequenceSet {
        nr_messages,
        start_end: if start.is_some() && end.is_some() {
            Some(StartEnd {
                start: start.unwrap(),
                end: end.unwrap(),
            })
        } else {
            None
        },
        idx: None,
    };

    match inbox_client.get_messages(session_id, mailbox, sequence_set) {
        Ok(messages) => {
            return format!(
                "{{\"success\": \"true\", \"message\": \"Messages retrieved\", \"data\": {}}}",
                messages
            )
        }
        Err(e) => {
            eprintln!("Error getting messages: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": \"{}\"}}", e);
        }
    }
}
