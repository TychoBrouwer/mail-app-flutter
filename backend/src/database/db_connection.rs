use async_std::sync::{Arc, Mutex};

use base64::{prelude::BASE64_STANDARD, Engine};
use rusqlite::{params, types::Value, vtab, Connection, OpenFlags};

use crate::my_error::MyError;
use crate::types::message::Message;
use crate::types::session::Client;

pub async fn new(database_path: &str) -> Result<Connection, MyError> {
    let conn = match Connection::open_with_flags(
        database_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    ) {
        Ok(conn) => conn,
        Err(e) => {
            
            let err = MyError::Sqlite(e, String::from("Error opening database"));
            err.log_error();

            return Err(err);
        }
    };

    match vtab::array::load_module(&conn) {
        Ok(_) => {}
        Err(e) => {
            
            let err = MyError::Sqlite(e, String::from("Error loading database array module"));
            err.log_error();

            return Err(err);
        }
    }

    return Ok(conn);
}

pub async fn initialise(conn: &Connection) -> Result<(), MyError> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS connections (
                username VARCHAR(500) NOT NULL,
                password VARCHAR(500) NOT NULL,
                address VARCHAR(500) NOT NULL,
                port INTEGER NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY(username, address)
            )",
        params![],
    ) {
        Ok(_) => {}
        Err(e) => {
            

            let err = MyError::Sqlite(e, String::from("Error creating connections table"));
            err.log_error();
            return Err(err);
        }
    }

    match conn.execute(
            "CREATE TABLE IF NOT EXISTS mailboxes (
                c_username VARCHAR(500) NOT NULL,
                c_address VARCHAR(500) NOT NULL,
                path VARCHAR(500) NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY(c_username, c_address, path),
                FOREIGN KEY(c_username, c_address) REFERENCES connections(username, address) ON DELETE CASCADE
            )",
            params![],
        ) {
            Ok(_) => {}
            Err(e) => {
                

                let err = MyError::Sqlite(e, String::from("Error creating mailboxes table"));
                err.log_error();

                return Err(err);
            }
        }

    match conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                message_uid INTEGER NOT NULL,
                c_username VARCHAR(500) NOT NULL,
                c_address VARCHAR(500) NOT NULL,
                m_path VARCHAR(500) NOT NULL,
                sequence_id INTEGER NULL,
                message_id VARCHAR(500) NOT NULL,
                subject VARCHAR(500) NOT NULL,
                from_ VARCHAR(500) NOT NULL,
                sender VARCHAR(500) NOT NULL,
                to_ VARCHAR(500) NOT NULL,
                cc VARCHAR(500) NOT NULL,
                bcc VARCHAR(500) NOT NULL,
                reply_to VARCHAR(500) NOT NULL,
                in_reply_to VARCHAR(500) NOT NULL,
                delivered_to VARCHAR(500) NOT NULL,
                date_ TIMESTAMP NOT NULL,
                received TIMESTAMP NOT NULL,
                flags VARCHAR(500) NOT NULL,
                html TEXT NOT NULL,
                text TEXT NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY(c_username, c_address, m_path, message_uid),
                FOREIGN KEY(c_username, c_address) REFERENCES connections(username, address) ON DELETE CASCADE,
                FOREIGN KEY(c_username, c_address, m_path) REFERENCES mailboxes(c_username, c_address, path) ON DELETE CASCADE
            )",
            params![],
        ) {
            Ok(_) => {}
            Err(e) => {
                

                let err = MyError::Sqlite(e, String::from("Error creating mailboxes table"));
                err.log_error();

                return Err(err);
            }
        }

    return Ok(());
}

pub async fn insert_connection(
    conn: Arc<Mutex<Connection>>,
    client: Client,
) -> Result<(), MyError> {
    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    match conn_locked.execute(
        "INSERT OR IGNORE INTO connections (
                username,
                password,
                address,
                port
            ) VALUES (?1, ?2, ?3, ?4)",
        params![
            client.username,
            client.password,
            client.address,
            client.port
        ],
    ) {
        Ok(_) => Ok({}),
        Err(e) => {
            

            let err = MyError::Sqlite(e, String::from("Error inserting connection into database"));
            err.log_error();

            return Err(err);
        }
    }
}

pub async fn insert_mailbox(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
) -> Result<(), MyError> {
    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    match conn_locked.execute(
        "INSERT OR IGNORE INTO mailboxes (
                c_username,
                c_address,
                path
            ) VALUES (?1, ?2, ?3)",
        params![username, address, mailbox_path],
    ) {
        Ok(_) => Ok({}),
        Err(e) => {
            

            let err = MyError::Sqlite(e, String::from("Error inserting mailbox into database"));
            err.log_error();

            return Err(err);
        }
    }
}

pub async fn insert_message(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    message: &Message,
) -> Result<(), MyError> {
    let html = match String::from_utf8(BASE64_STANDARD.decode(message.html.as_str()).unwrap()) {
        Ok(html) => html,
        Err(e) => {
            

            let err = MyError::FromUtf8(e, String::from("Error decoding HTML for database"));
            err.log_error();

            return Err(err);
        }
    };

    let decode_text = match BASE64_STANDARD.decode(message.text.as_str()) {
        Ok(decode) => decode,
        Err(e) => {
            

            let err = MyError::Base64(e, String::from("Error decoding text for database"));
            err.log_error();

            return Err(err);
        }
    };

    let text = match String::from_utf8(decode_text) {
        Ok(text) => text,
        Err(e) => {
            

            let err = MyError::FromUtf8(e, String::from("Error decoding text bytes for database"));
            err.log_error();

            return Err(err);
        }
    };

    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    match conn_locked.execute(
            "INSERT OR IGNORE INTO messages (
                message_uid,
                c_username,
                c_address,
                m_path,
                sequence_id,
                message_id,
                subject,
                from_,
                sender,
                to_,
                cc,
                bcc,
                reply_to,
                in_reply_to,
                delivered_to,
                date_,
                received,
                flags,
                html,
                text
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
            params![
                message.message_uid,
                username,
                address,
                mailbox_path,
                message.sequence_id,
                message.message_id,
                message.subject,
                message.from,
                message.sender,
                message.to,
                message.cc,
                message.bcc,
                message.reply_to,
                message.in_reply_to,
                message.delivered_to,
                message.date,
                message.received,
                message.flags,
                html,
                text
            ],
        ) {
            Ok(_) => Ok({}),
            Err(e) => {
                

                let err = MyError::Sqlite(e, String::from("Error inserting message into database"));
                err.log_error();

                return Err(err);
            }
        }
}

pub async fn update_message_flags(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    message_uid: u32,
    flags_str: &str,
) -> Result<(), MyError> {
    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    match conn_locked.execute(
        "UPDATE messages
             SET flags = ?1
             WHERE message_uid = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
        params![flags_str, message_uid, username, address, mailbox_path],
    ) {
        Ok(_) => Ok({}),
        Err(e) => {
            
            let err = MyError::Sqlite(e, String::from("Error updating flags in database"));
            err.log_error();

            return Err(err);
        }
    }
}

pub async fn move_message(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    message_uid: u32,
    mailbox_path_dest: &str,
) -> Result<(), MyError> {
    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    match conn_locked.execute(
        "UPDATE messages
             SET m_path = ?1
             WHERE message_uid = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
        params![
            mailbox_path_dest,
            message_uid,
            username,
            address,
            mailbox_path
        ],
    ) {
        Ok(_) => Ok({}),
        Err(e) => {
            
            let err = MyError::Sqlite(e, String::from("Error moving message in database"));
            err.log_error();

            return Err(err);
        }
    }
}

pub async fn update_message_sequence_id(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    message_uid: u32,
    sequence_id: u32,
) -> Result<(), MyError> {
    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    match conn_locked.execute(
        "UPDATE messages
             SET sequence_id = NULL
             WHERE sequence_id = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
        params![sequence_id, username, address, mailbox_path],
    ) {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error updating sequence id in database"));
            err.log_error();

            return Err(err);
        }
    };

    match conn_locked.execute(
        "UPDATE messages
             SET sequence_id = ?1
             WHERE message_uid = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
        params![sequence_id, message_uid, username, address, mailbox_path],
    ) {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(
                e,
                String::from("Error clearing sequence id column in database"),
            );
            err.log_error();

            return Err(err);
        }
    }

    return Ok(());
}

pub async fn get_connections(conn: Arc<Mutex<Connection>>) -> Result<Vec<Client>, MyError> {
    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    let mut stmt = match conn_locked.prepare("SELECT * FROM connections") {
        Ok(stmt) => stmt,
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error preparing statement at connections"));
            err.log_error();

            return Err(err);
        }
    };

    match stmt.query_map(params![], |row| {
        Ok(Client {
            username: row.get(0).unwrap(),
            password: row.get(1).unwrap(),
            address: row.get(2).unwrap(),
            port: row.get(3).unwrap(),
        })
    }) {
        Ok(rows) => {
            let mut connections: Vec<Client> = Vec::new();

            for row in rows {
                connections.push(match row {
                    Ok(session) => session,
                    Err(_) => continue,
                });
            }

            return Ok(connections);
        }
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error getting connections from database"));
            err.log_error();

            return Err(err);
        }
    };
}

pub async fn get_mailboxes(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
) -> Result<Vec<String>, MyError> {
    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    let mut stmt = match conn_locked
        .prepare("SELECT * FROM mailboxes WHERE c_username = ?1 AND c_address = ?2")
    {
        Ok(stmt) => stmt,
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error preparing statement at mailboxes"));
            err.log_error();

            return Err(err);
        }
    };

    let mut mailboxes: Vec<String> = Vec::new();

    match stmt.query_map(params![username, address], |row| row.get(2)) {
        Ok(rows) => {
            for row in rows {
                mailboxes.push(row.unwrap());
            }
        }
        Err(e) => {
            
            let err = MyError::Sqlite(e, String::from("Error getting mailboxes from database"));
            err.log_error();

            return Err(err);
        }
    }

    return Ok(mailboxes);
}

pub async fn get_messages_with_uids(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    message_uids: &Vec<u32>,
) -> Result<Vec<Message>, MyError> {
    let uid_list: vtab::array::Array = std::rc::Rc::new(
        message_uids
            .into_iter()
            .map(|uid| Value::from(*uid))
            .collect::<Vec<Value>>(),
    );

    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    let mut stmt = match conn_locked.prepare(
            "SELECT * FROM messages WHERE message_uid IN rarray(?1) AND c_username = ?2 AND c_address = ?3 AND m_path = ?4",
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                
            let err = MyError::Sqlite(e, String::from("Error preparing statement at messages with uids"));
            err.log_error();

            return Err(err);
        }
    };

    match stmt.query_map(params![uid_list, username, address, mailbox_path], |row| {
        Ok(Message::from_row(row))
    }) {
        Ok(messages) => {
            let mut messages_list: Vec<Message> = Vec::new();

            for message in messages {
                match message {
                    Ok(message) => messages_list.push(message),
                    Err(_) => {
                        
                        continue;
                    }
                }
            }

            return Ok(messages_list);
        }
        Err(e) => {
            
            let err = MyError::Sqlite(e, String::from("Error getting message from database"));
            err.log_error();

            return Err(err);
        }
    };
}

pub async fn get_messages_sorted(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    start: u32,
    end: u32,
) -> Result<Vec<Message>, MyError> {
    let limit = end - start + 1;

    let conn_locked = conn.lock().await;
    dbg!("locked conn");

    let mut stmt = match conn_locked.prepare(
            "SELECT * FROM messages WHERE c_username = ?1 AND c_address = ?2 AND m_path = ?3 ORDER BY received DESC LIMIT ?4 OFFSET ?5",
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                
                let err = MyError::Sqlite(e, String::from("Error preparing statement at messages"));
                err.log_error();

                return Err(err);
            }
        };

    match stmt.query_map(
        params![username, address, mailbox_path, limit, start],
        |row| Ok(Message::from_row(row)),
    ) {
        Ok(messages) => {
            let mut messages_list: Vec<Message> = Vec::new();

            for message in messages {
                match message {
                    Ok(message) => messages_list.push(message),
                    Err(_) => {
                        
                        continue;
                    }
                }
            }

            return Ok(messages_list);
        }
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error getting message from database"));

            err.log_error();
            return Err(err);
        }
    };
}
