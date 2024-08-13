use async_std::sync::{Arc, Mutex};

use crate::http_server::params;
use crate::inbox_client;
use crate::types::session::{Client, Session};

pub async fn login(
    uri: &str,
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
) -> String {
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

    let mut locked_clients = clients.lock().await;
    dbg!("locked clients");
    match locked_clients
        .iter()
        .position(|x| x.username == username.to_string() && x.address == address.to_string())
    {
        Some(idx) => {
            return format!("{{\"success\": true, \"message\": \"Allready connected to IMAP server\", \"data\": {{ \"session_id\": {}}}}}", idx);
        }
        None => {}
    };

    locked_clients.push(Client {
        username: username.to_string(),
        password: password.to_string(),
        address: address.to_string(),
        port,
    });

    let idx = locked_clients.len() - 1;

    drop(locked_clients);

    match inbox_client::connect::connect(sessions, database_conn, clients, idx).await {
        Ok(idx) => {
            return format!("{{\"success\": true, \"message\": \"Connected to IMAP server\", \"data\": {{ \"session_id\": {}}}}}", idx);
        }
        Err(e) => {
            eprintln!("Error connecting to IMAP server: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

pub async fn logout(
    uri: &str,
    sessions: Arc<Mutex<Vec<Session>>>,
    clients: Arc<Mutex<Vec<Client>>>,
) -> String {
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

    let mut locked_clients = clients.lock().await;
    dbg!("locked clients");

    if session_id + 1 > locked_clients.len() {
        return String::from("{\"success\": false, \"message\": \"Invalid session_id\"}");
    }

    match inbox_client::logout::logout_imap(sessions, session_id).await {
        Ok(_) => {
            locked_clients.remove(session_id);
            return String::from("{\"success\": true, \"message\": \"Logged out\"}");
        }
        Err(e) => {
            eprintln!("Error logging out: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

pub async fn get_sessions(clients: Arc<Mutex<Vec<Client>>>) -> String {
    let mut response =
        String::from("{\"success\": true, \"message\": \"Sessions retrieved\", \"data\": [");

    let locked_clients = clients.lock().await;
    dbg!("locked clients");

    for (i, client) in locked_clients.iter().enumerate() {
        response.push_str(&format!(
            "{{\"session_id\": {}, \"username\": \"{}\", \"address\": \"{}\", \"port\": {}}}",
            i, client.username, client.address, client.port
        ));

        if i < locked_clients.len() - 1 {
            response.push_str(",");
        }
    }
    response.push_str("]}");

    return response;
}

pub async fn get_mailboxes(
    uri: &str,
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
) -> String {
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

    match inbox_client::get_mailboxes::get_mailboxes(sessions, database_conn, session_id, clients)
        .await
    {
        Ok(mailboxes) => {
            return format!(
                "{{\"success\": true, \"message\": \"Mailboxes retrieved\", \"data\": {}}}",
                mailboxes
            )
        }
        Err(e) => {
            eprintln!("Error getting mailboxes: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

pub async fn get_messages_with_uids(
    uri: &str,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
) -> String {
    let uri_params = params::parse_params(String::from(uri));

    let session_id = match params::get_usize(uri_params.get("session_id")) {
        Ok(session_id) => session_id,
        Err(e) => {
            eprintln!("Error parsing session_id: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    };
    let mailbox_path = uri_params.get("mailbox_path");
    let message_uids = uri_params.get("message_uids");

    if session_id.is_none() || mailbox_path.is_none() || message_uids.is_none() {
        eprintln!(
            "Provide session_id, mailbox_path and message_id GET parameters: {}",
            uri
        );
        return String::from("{\"success\": false, \"message\": \"Provide session_id, mailbox_path and message_uid GET parameters\"}");
    }

    let session_id = session_id.unwrap();
    let mailbox_path = mailbox_path.unwrap();
    let message_uids = message_uids.unwrap();

    let message_uids: Vec<u32> = message_uids
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    match inbox_client::get_messages_with_uids::get_messages_with_uids(
        database_conn,
        session_id,
        clients,
        mailbox_path,
        &message_uids,
    )
    .await
    {
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

pub async fn get_messages_sorted(
    uri: &str,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
) -> String {
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

    match inbox_client::get_messages_sorted::get_messages_sorted(
        database_conn,
        session_id,
        clients,
        mailbox_path,
        start,
        end,
    )
    .await
    {
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

pub async fn update_mailbox(
    uri: &str,
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
) -> String {
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

    match inbox_client::update_mailbox::update_mailbox(
        sessions,
        database_conn,
        session_id,
        clients,
        mailbox_path,
    )
    .await
    {
        Ok(message) => {
            return format!(
                "{{\"success\": true, \"message\": \"Mailbox updated\", \"data\": {}}}",
                message
            );
        }
        Err(e) => {
            eprintln!("Error updating mailbox: {:?}", e);
            return format!("{{\"success\": false, \"message\": \"{}\"}}", e);
        }
    }
}

pub async fn modify_flags(
    uri: &str,
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
) -> String {
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

    match inbox_client::modify_flags::modify_flags(
        sessions,
        database_conn,
        session_id,
        clients,
        mailbox_path,
        message_uid,
        flags,
        add,
    )
    .await
    {
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

pub async fn move_message(
    uri: &str,
    sessions: Arc<Mutex<Vec<Session>>>,
    database_conn: Arc<Mutex<rusqlite::Connection>>,
    clients: Arc<Mutex<Vec<Client>>>,
) -> String {
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

    match inbox_client::move_message::move_message(
        sessions,
        database_conn,
        session_id,
        clients,
        mailbox_path,
        message_uid,
        mailbox_path_dest,
    )
    .await
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
