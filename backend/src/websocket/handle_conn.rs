use crate::inbox_client::inbox_client as inbox_client;
use crate::websocket::params as params;

pub fn login(uri: &str, inbox_client: &mut inbox_client::InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let email = uri_params.get("email");
    let password = uri_params.get("password");
    let address = uri_params.get("address");
    let port = params::get_u16(uri_params.get("port"));

    if email.is_none() || password.is_none() || address.is_none() || port.is_none() {
        eprintln!("Provide all GET parameters: {}", uri);
        return String::from("{\"success\": \"false\", \"message\": \"Provide all GET parameters\"}");
    }

    let email = email.unwrap();
    let password = password.unwrap();
    let address = address.unwrap();
    let port = port.unwrap();

    if inbox_client.addresses.contains(&email.to_string()) {
        let idx = inbox_client.addresses.iter().position(|x| x == email).unwrap();
        
        return format!("{{\"success\": \"true\", \"message\": \"Allready connected to IMAP server\", \"data\": {{ \"id\": {}}}}}", idx)
    }

    match inbox_client.connect(address.as_str(), port, email.as_str(), password.as_str()) {
        Ok(idx) => {
            format!("{{\"success\": \"true\", \"message\": \"Connected to IMAP server\", \"data\": {{ \"id\": {}}}}}", idx)
        },
        Err(e) => {
            eprintln!("Error connecting to IMAP server: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": \"{}\"}}", e);
        }
    }   
}

pub fn logout(uri: &str, inbox_client: &mut inbox_client::InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));
    
    let session_id = params::get_usize(uri_params.get("session_id"));

    if session_id.is_none() {
        eprintln!("Provide session_id GET parameter: {}", uri);
        return String::from("{\"success\": \"false\", \"message\": \"Provide session_id GET parameter\"}");
    }

    let session_id = session_id.unwrap();

    match inbox_client.logout(session_id) {
        Ok(_) => {
            return String::from("{\"success\": \"true\", \"message\": \"Logged out\"}");
        },
        Err(e) => {
            eprintln!("Error logging out: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": \"{}\"}}", e);
        }
    }
}

pub fn message_envelopes(uri: &str, inbox_client: &mut inbox_client::InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = params::get_usize(uri_params.get("session_id"));
    let mailbox = uri_params.get("mailbox");

    let nr_messages = params::get_usize(uri_params.get("nr_messages"));
    let start = params::get_usize(uri_params.get("start"));
    let end = params::get_usize(uri_params.get("end"));

    if session_id.is_none() || mailbox.is_none() || (nr_messages.is_none() && (start.is_none() || end.is_none())) {
        eprintln!("Provide session_id GET parameter: {}", uri);
        return String::from("{\"success\": \"false\", \"message\": \"Provide session_id GET parameter\"}");
    }

    let session_id = session_id.unwrap();
    let mailbox = mailbox.unwrap();
    let sequence_set = inbox_client::SequenceSet {
        nr_messages,
        start_end: if start.is_some() && end.is_some() {
            Some(inbox_client::StartEnd {
                start: start.unwrap(),
                end: end.unwrap(),
            })
        } else {
            None
        }
    };
    
    match inbox_client.get_messages(session_id, mailbox.to_string(), sequence_set) {
        Ok(messages) => {
            return format!("{{\"success\": \"true\", \"message\": \"Message envelopes retrieved\", \"data\": {}}}", messages)
        },
        Err(e) => {
            eprintln!("Error getting message envelopes: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": \"{}\"}}", e);
        }
    }
}

pub fn message(uri: &str, inbox_client: &mut inbox_client::InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = params::get_usize(uri_params.get("session_id"));
    let mailbox = uri_params.get("mailbox");
    let message_uid = params::get_u32(uri_params.get("message_uid"));

    if session_id.is_none() || mailbox.is_none() || message_uid.is_none() {
        eprintln!("Provide session_id, mailbox and message_id GET parameters: {}", uri);
        return String::from("{\"success\": \"false\", \"message\": \"Provide session_id, mailbox and message_uid GET parameters\"}");
    }

    let session_id = session_id.unwrap();
    let mailbox = mailbox.unwrap();
    let message_uid = message_uid.unwrap();

    match inbox_client.get_message(session_id, mailbox.to_string(), message_uid) {
        Ok(message) => {
            return format!("{{\"success\": \"true\", \"message\": \"Message retrieved\", \"data\": {}}}", message)
        },
        Err(e) => {
            eprintln!("Error getting message: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": \"{}\"}}", e);
        }
    }
}

pub fn mailboxes(uri: &str, inbox_client: &mut inbox_client::InboxClient) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = params::get_usize(uri_params.get("session_id"));

    if session_id.is_none() {
        eprintln!("Provide session_id GET parameter: {}", uri);
        return String::from("{\"success\": \"false\", \"message\": \"Provide session_id GET parameter\"}");
    }

    let session_id = session_id.unwrap();

    match inbox_client.get_all_mailboxes(session_id) {
        Ok(mailboxes) => {
            return format!("{{\"success\": \"true\", \"message\": \"Mailboxes retrieved\", \"data\": {}}}", mailboxes)
        },
        Err(e) => {
            eprintln!("Error getting mailboxes: {:?}", e);
            return format!("{{\"success\": \"false\", \"message\": \"{}\"}}", e);
        }
    }
}    
