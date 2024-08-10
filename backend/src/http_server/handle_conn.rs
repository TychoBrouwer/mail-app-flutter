use std::sync::{Arc, Mutex};

use crate::{
    http_server::params,
    inbox_client::inbox_client::{InboxClient, Session},
};

pub fn login(uri: &str, inbox_client: Arc<Mutex<InboxClient>>) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let username = uri_params.get("username");
    let password = uri_params.get("password");
    let address = uri_params.get("address");
    let port = match params::get_u16(uri_params.get("port")) {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Error parsing port: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };

    if username.is_none() || password.is_none() || address.is_none() || port.is_none() {
        eprintln!("Provide all GET parameters: {}", uri);
        return String::from("{\"success\": false, \"message\": \"Provide all GET parameters\"}");
    }

    let username = username.unwrap();
    let password = password.unwrap();
    let address = address.unwrap();
    let port = port.unwrap();

    let mut locked_inbox_client = inbox_client.lock().unwrap();
    match locked_inbox_client
        .sessions
        .iter()
        .position(|x| x.username == username.to_string() && x.address == address.to_string())
    {
        Some(idx) => {
            return format!("{{\"success\": true, \"message\": \"Allready connected to IMAP server\", \"data\": {{ \"session_id\": {}}}}}", idx);
        }
        None => {}
    };

    match locked_inbox_client.connect(Session {
        stream: None,
        username: username.to_string(),
        password: password.to_string(),
        address: address.to_string(),
        port,
    }) {
        Ok(idx) => {
            format!("{{\"success\": true, \"message\": \"Connected to IMAP server\", \"data\": {{ \"session_id\": {}}}}}", idx)
        }
        Err(e) => {
            eprintln!("Error connecting to IMAP server: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

pub fn logout(uri: &str, inbox_client: Arc<Mutex<InboxClient>>) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = match params::get_usize(uri_params.get("session_id")) {
        Ok(session_id) => session_id,
        Err(e) => {
            eprintln!("Error parsing session_id: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };

    if session_id.is_none() {
        eprintln!("Provide session_id GET parameter: {}", uri);
        return String::from(
            "{\"success\": false, \"message\": \"Provide session_id GET parameter\"}",
        );
    }

    let session_id = session_id.unwrap();

    let mut locked_inbox_client = inbox_client.lock().unwrap();
    match locked_inbox_client.logout_imap(session_id) {
        Ok(_) => {
            return String::from("{\"success\": true, \"message\": \"Logged out\"}");
        }
        Err(e) => {
            eprintln!("Error logging out: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

pub fn get_sessions(inbox_client: Arc<Mutex<InboxClient>>) -> String {
    let locked_inbox_client = inbox_client.lock().unwrap();
    let sessions = &locked_inbox_client.sessions;

    let mut response =
        String::from("{\"success\": true, \"message\": \"Sessions retrieved\", \"data\": [");
    for (i, session) in sessions.iter().enumerate() {
        response.push_str(&format!(
            "{{\"session_id\": {}, \"username\": \"{}\", \"address\": \"{}\", \"port\": {}}}",
            i, session.username, session.address, session.port
        ));

        if i < sessions.len() - 1 {
            response.push_str(",");
        }
    }
    response.push_str("]}");

    return response;
}

pub fn get_mailboxes(uri: &str, inbox_client: Arc<Mutex<InboxClient>>) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = match params::get_usize(uri_params.get("session_id")) {
        Ok(session_id) => session_id,
        Err(e) => {
            eprintln!("Error parsing session_id: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };

    if session_id.is_none() {
        eprintln!("Provide session_id GET parameter: {}", uri);
        return String::from(
            "{\"success\": false, \"message\": \"Provide session_id GET parameter\"}",
        );
    }

    let session_id = session_id.unwrap();

    let mut locked_inbox_client = inbox_client.lock().unwrap();
    match locked_inbox_client.get_mailboxes(session_id) {
        Ok(mailbox_pathes) => {
            return format!(
                "{{\"success\": true, \"message\": \"Mailboxes retrieved\", \"data\": {}}}",
                mailbox_pathes
            )
        }
        Err(e) => {
            eprintln!("Error getting mailbox_pathes: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

// pub fn get_message(uri: &str, inbox_client: Arc<Mutex<InboxClient>>) -> String {
//     let uri_params = params::parse_params(String::from(uri));

//     let session_id = match params::get_usize(uri_params.get("session_id")) {
//         Ok(session_id) => session_id,
//         Err(e) => {
//             eprintln!("Error parsing session_id: {:?}", e);
//             return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
//         }
//     };
//     let mailbox_path = uri_params.get("mailbox_path");
//     let message_uid = match params::get_u32(uri_params.get("message_uid")) {
//         Ok(message_uid) => message_uid,
//         Err(e) => {
//             eprintln!("Error parsing message_uid: {:?}", e);
//             return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
//         }
//     };

//     if session_id.is_none() || mailbox_path.is_none() || message_uid.is_none() {
//         eprintln!(
//             "Provide session_id, mailbox_path and message_id GET parameters: {}",
//             uri
//         );
//         return String::from("{\"success\": false, \"message\": \"Provide session_id, mailbox_path and message_uid GET parameters\"}");
//     }

//     let session_id = session_id.unwrap();
//     let mailbox_path = mailbox_path.unwrap();
//     let message_uid = message_uid.unwrap();

//     let mut locked_inbox_client = inbox_client.lock().unwrap();
//     match locked_inbox_client.get_message(session_id, mailbox_path, message_uid) {
//         Ok(message) => {
//             return format!(
//                 "{{\"success\": true, \"message\": \"Message retrieved\", \"data\": {}}}",
//                 message
//             )
//         }
//         Err(e) => {
//             eprintln!("Error getting message: {:?}", e);
//             return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
//         }
//     }
// }

pub fn get_messages_sorted(uri: &str, inbox_client: Arc<Mutex<InboxClient>>) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = match params::get_usize(uri_params.get("session_id")) {
        Ok(session_id) => session_id,
        Err(e) => {
            eprintln!("Error parsing session_id: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };
    let mailbox_path = uri_params.get("mailbox_path");

    let start = match params::get_u32(uri_params.get("start")) {
        Ok(start) => start,
        Err(e) => {
            eprintln!("Error parsing start: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };
    let end = match params::get_u32(uri_params.get("end")) {
        Ok(end) => end,
        Err(e) => {
            eprintln!("Error parsing end: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };

    if session_id.is_none() || mailbox_path.is_none() || start.is_none() || end.is_none() {
        eprintln!("Provide session_id GET parameter: {}", uri);
        return String::from(
            "{\"success\": false, \"message\": \"Provide session_id, mailbox_path, start, and end GET parameters\"}",
        );
    }

    let session_id = session_id.unwrap();
    let mailbox_path = mailbox_path.unwrap();
    let start = start.unwrap();
    let end = end.unwrap();

    let mut locked_inbox_client = inbox_client.lock().unwrap();
    match locked_inbox_client.get_messages_sorted(session_id, mailbox_path, start, end) {
        Ok(messages) => {
            return format!(
                "{{\"success\": true, \"message\": \"Messages retrieved\", \"data\": {}}}",
                messages
            )
        }
        Err(e) => {
            eprintln!("Error getting messages: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

pub fn update_mailbox(uri: &str, inbox_client: Arc<Mutex<InboxClient>>) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = match params::get_usize(uri_params.get("session_id")) {
        Ok(session_id) => session_id,
        Err(e) => {
            eprintln!("Error parsing session_id: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };
    let mailbox_path = uri_params.get("mailbox_path");

    if session_id.is_none() || mailbox_path.is_none() {
        eprintln!(
            "Provide session_id and mailbox_path GET parameters: {}",
            uri
        );
        return String::from("{\"success\": false, \"message\": \"Provide session_id and mailbox_path GET parameters\"}");
    }

    let session_id = session_id.unwrap();
    let mailbox_path = mailbox_path.unwrap();

    let mut locked_inbox_client = inbox_client.lock().unwrap();
    match locked_inbox_client.update_mailbox(session_id, mailbox_path) {
        Ok(messages) => {
            return format!(
                "{{\"success\": true, \"message\": \"Mailbox updated\", \"data\": {}}}",
                messages
            )
        }
        Err(e) => {
            eprintln!("Error updating mailbox: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

pub fn modify_flags(uri: &str, inbox_client: Arc<Mutex<InboxClient>>) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = match params::get_usize(uri_params.get("session_id")) {
        Ok(session_id) => session_id,
        Err(e) => {
            eprintln!("Error parsing session_id: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };
    let mailbox_path = uri_params.get("mailbox_path");
    let message_uid = match params::get_u32(uri_params.get("message_uid")) {
        Ok(message_uid) => message_uid,
        Err(e) => {
            eprintln!("Error parsing message_uid: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };
    let flags = uri_params.get("flags");
    let add = match params::get_bool(uri_params.get("add")) {
        Ok(add) => add,
        Err(e) => {
            eprintln!("Error parsing add: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };

    if session_id.is_none()
        || mailbox_path.is_none()
        || message_uid.is_none()
        || flags.is_none()
        || add.is_none()
    {
        eprintln!(
            "Provide session_id, mailbox_path, message_id, flags, and add GET parameters: {}",
            uri
        );
        return String::from("{\"success\": false, \"message\": \"Provide session_id, mailbox_path, message_uid, flags, and add GET parameters\"}");
    }

    let session_id = session_id.unwrap();
    let mailbox_path = mailbox_path.unwrap();
    let message_uid = message_uid.unwrap();
    let flags = flags.unwrap();
    let add = add.unwrap();

    let mut locked_inbox_client = inbox_client.lock().unwrap();
    match locked_inbox_client.modify_flags(session_id, mailbox_path, message_uid, flags, add) {
        Ok(message) => {
            return format!(
                "{{\"success\": true, \"message\": \"Flags successfully updated\", \"data\": {}}}",
                message
            )
        }
        Err(e) => {
            eprintln!("Error updating flags: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

pub fn move_message(uri: &str, inbox_client: Arc<Mutex<InboxClient>>) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = match params::get_usize(uri_params.get("session_id")) {
        Ok(session_id) => session_id,
        Err(e) => {
            eprintln!("Error parsing session_id: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };
    let mailbox_path = uri_params.get("mailbox_path");
    let message_uid = match params::get_u32(uri_params.get("message_uid")) {
        Ok(message_uid) => message_uid,
        Err(e) => {
            eprintln!("Error parsing message_uid: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };
    let mailbox_path_dest = uri_params.get("mailbox_path_dest");

    if session_id.is_none()
        || mailbox_path.is_none()
        || message_uid.is_none()
        || mailbox_path_dest.is_none()
    {
        eprintln!(
            "Provide session_id, mailbox_path, message_id, and mailbox_path_dest GET parameters: {}",
            uri
        );
        return String::from("{\"success\": false, \"message\": \"Provide session_id, mailbox_path, message_uid, and mailbox_path_dest GET parameters\"}");
    }

    let session_id = session_id.unwrap();
    let mailbox_path = mailbox_path.unwrap();
    let message_uid = message_uid.unwrap();
    let mailbox_path_dest = mailbox_path_dest.unwrap();

    let mut locked_inbox_client = inbox_client.lock().unwrap();
    match locked_inbox_client.move_message(session_id, mailbox_path, message_uid, mailbox_path_dest)
    {
        Ok(message) => {
            return format!(
                "{{\"success\": true, \"message\": \"Message successfully moved\", \"data\": {}}}",
                message
            )
        }
        Err(e) => {
            eprintln!("Error moving message: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}
