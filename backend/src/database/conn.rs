use crate::inbox_client::{inbox_client::Session, parse_message::Message};
use crate::my_error::MyError;

use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::offset;
use rusqlite::{params, vtab, Connection, OpenFlags};

pub struct DBConnection {
    conn: Connection,
}

impl DBConnection {
    pub fn new(database_path: &str) -> Result<DBConnection, MyError> {
        let conn = match Connection::open_with_flags(
            database_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        ) {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Error opening database: {}", e);
                return Err(MyError::Sqlite(e));
            }
        };

        match vtab::array::load_module(&conn) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error loading array module: {}", e);
                return Err(MyError::Sqlite(e));
            }
        }

        return Ok(DBConnection { conn });
    }

    pub fn initialise(&mut self) -> Result<(), MyError> {
        match self.conn.execute(
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
                eprintln!("Error creating connections table: {}", e);

                return Err(MyError::Sqlite(e));
            }
        }

        match self.conn.execute(
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
                eprintln!("Error creating mailboxes table: {}", e);

                return Err(MyError::Sqlite(e));
            }
        }

        match self.conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                uid INTEGER NOT NULL,
                c_username VARCHAR(500) NOT NULL,
                c_address VARCHAR(500) NOT NULL,
                m_path VARCHAR(500) NOT NULL,
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
                PRIMARY KEY(c_username, c_address, m_path, uid),
                FOREIGN KEY(c_username, c_address) REFERENCES connections(username, address) ON DELETE CASCADE,
                FOREIGN KEY(c_username, c_address, m_path) REFERENCES mailboxes(c_username, c_address, path) ON DELETE CASCADE
            )",
            params![],
        ) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error creating mailboxes table: {}", e);

                return Err(MyError::Sqlite(e));
            }
        }

        return Ok(());
    }

    pub fn insert_connection(&mut self, session: &Session) -> Result<(), MyError> {
        match self.conn.execute(
            "INSERT OR IGNORE INTO connections (
                username,
                password,
                address,
                port
            ) VALUES (?1, ?2, ?3, ?4)",
            params![
                session.username,
                session.password,
                session.address,
                session.port
            ],
        ) {
            Ok(_) => Ok({}),
            Err(e) => {
                eprintln!("Error inserting connection: {}", e);

                return Err(MyError::Sqlite(e));
            }
        }
    }

    pub fn insert_mailbox(
        &mut self,
        username: &str,
        address: &str,
        mailbox_path: &str,
    ) -> Result<(), MyError> {
        match self.conn.execute(
            "INSERT OR IGNORE INTO mailboxes (
                c_username,
                c_address,
                path
            ) VALUES (?1, ?2, ?3)",
            params![username, address, mailbox_path],
        ) {
            Ok(_) => Ok({}),
            Err(e) => {
                eprintln!("Error inserting mailbox: {}", e);

                return Err(MyError::Sqlite(e));
            }
        }
    }

    pub fn insert_message(
        &mut self,
        username: &str,
        address: &str,
        mailbox_path: &str,
        message: &Message,
    ) -> Result<(), MyError> {
        let html = match String::from_utf8(BASE64_STANDARD.decode(message.html.as_str()).unwrap()) {
            Ok(html) => html,
            Err(e) => {
                eprintln!("Error decoding HTML: {}", e);

                return Err(MyError::FromUtf8(e));
            }
        };

        let decode_text = match BASE64_STANDARD.decode(message.text.as_str()) {
            Ok(decode) => decode,
            Err(e) => {
                eprintln!("Error decoding text: {}", e);

                return Err(MyError::Base64(e));
            }
        };

        let text = match String::from_utf8(decode_text) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Error decoding text bytes: {}", e);

                return Err(MyError::FromUtf8(e));
            }
        };

        match self.conn.execute(
            "INSERT OR IGNORE INTO messages (
                uid,
                c_username,
                c_address,
                m_path,
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
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
            params![
                message.uid,
                username,
                address,
                mailbox_path,
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
                eprintln!("Error inserting message: {}", e);

                return Err(MyError::Sqlite(e));
            }
        }
    }

    pub fn update_message_flags(
        &mut self,
        username: &str,
        address: &str,
        mailbox_path: &str,
        message_uid: u32,
        flags: &str,
    ) -> Result<(), MyError> {
        match self.conn.execute(
            "UPDATE messages
             SET flags = ?1
             WHERE uid = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
            params![flags, message_uid, username, address, mailbox_path],
        ) {
            Ok(_) => Ok({}),
            Err(e) => {
                eprintln!("Error updating flags column: {}", e);
                return Err(MyError::Sqlite(e));
            }
        }
    }

    pub fn move_message(
        &mut self,
        username: &str,
        address: &str,
        mailbox_path: &str,
        message_uid: u32,
        mailbox_path_dest: &str,
    ) -> Result<(), MyError> {
        match self.conn.execute(
            "UPDATE messages
             SET m_path = ?1
             WHERE uid = ?2 AND c_username = ?3 AND c_address = ?4 AND m_path = ?5",
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
                eprintln!("Error moving message: {}", e);
                return Err(MyError::Sqlite(e));
            }
        }
    }

    pub fn get_connections(&mut self) -> Result<Vec<Session>, MyError> {
        let mut stmt = match self.conn.prepare("SELECT * FROM connections") {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement at connections: {}", e);
                return Err(MyError::Sqlite(e));
            }
        };

        match stmt.query_map(params![], |row| {
            Ok(Session {
                stream: None,
                username: row.get(0).unwrap(),
                password: row.get(1).unwrap(),
                address: row.get(2).unwrap(),
                port: row.get(3).unwrap(),
            })
        }) {
            Ok(rows) => {
                let mut connections: Vec<Session> = Vec::new();

                for row in rows {
                    connections.push(match row {
                        Ok(session) => session,
                        Err(_) => continue,
                    });
                }

                return Ok(connections);
            }
            Err(e) => {
                eprintln!("Error getting connections: {}", e);
                return Err(MyError::Sqlite(e));
            }
        };
    }

    pub fn get_mailboxes(&mut self, username: &str, address: &str) -> Result<Vec<String>, MyError> {
        let mut stmt = match self
            .conn
            .prepare("SELECT * FROM mailboxes WHERE c_username = ?1 AND c_address = ?2")
        {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement at mailboxes: {}", e);
                return Err(MyError::Sqlite(e));
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
                eprintln!("Error getting mailboxes: {}", e);
                return Err(MyError::Sqlite(e));
            }
        }

        return Ok(mailboxes);
    }

    pub fn get_messages_with_uid(
        &mut self,
        username: &str,
        address: &str,
        mailbox_path: &str,
        uid: &Vec<u32>,
    ) -> Result<Vec<Message>, MyError> {
        let mut stmt = match self.conn.prepare(
            "SELECT * FROM messages WHERE uid IN rarray(?1) AND c_username = ?2 AND c_address = ?3 AND m_path = ?4",
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement: {}", e);
                return Err(MyError::Sqlite(e));
            }
        };

        let uid_list: vtab::array::Array = std::rc::Rc::new(
            uid.into_iter()
                .map(|uid| rusqlite::types::Value::from(*uid))
                .collect::<Vec<rusqlite::types::Value>>(),
        );

        match stmt.query_map(params![uid_list, username, address, mailbox_path], |row| {
            let html: String = row.get(17).unwrap();
            let text: String = row.get(18).unwrap();

            Ok(Message {
                uid: row.get(0).unwrap(),
                message_id: row.get(4).unwrap(),
                subject: row.get(5).unwrap(),
                from: row.get(6).unwrap(),
                sender: row.get(7).unwrap(),
                to: row.get(8).unwrap(),
                cc: row.get(9).unwrap(),
                bcc: row.get(10).unwrap(),
                reply_to: row.get(11).unwrap(),
                in_reply_to: row.get(12).unwrap(),
                delivered_to: row.get(13).unwrap(),
                date: row.get(14).unwrap(),
                received: row.get(15).unwrap(),
                flags: row.get(16).unwrap(),
                html: BASE64_STANDARD.encode(html.as_bytes()),
                text: BASE64_STANDARD.encode(text.as_bytes()),
            })
        }) {
            Ok(messages) => {
                let mut messages_list: Vec<Message> = Vec::new();

                for message in messages {
                    match message {
                        Ok(message) => messages_list.push(message),
                        Err(_) => {
                            eprintln!("Message not present in local database");
                            continue;
                        }
                    }
                }

                return Ok(messages_list);
            }
            Err(e) => {
                eprintln!("Error getting message: {}", e);
                return Err(MyError::Sqlite(e));
            }
        };
    }

    pub fn get_messages_sorted(
        &mut self,
        username: &str,
        address: &str,
        mailbox_path: &str,
        start: u32,
        end: u32,
    ) -> Result<Vec<Message>, MyError> {
        let mut stmt = match self.conn.prepare(
            "SELECT * FROM messages WHERE c_username = ?1 AND c_address = ?2 AND m_path = ?3 ORDER BY received DESC LIMIT ?4 OFFSET ?5",
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement: {}", e);
                return Err(MyError::Sqlite(e));
            }
        };

        let limit = end - start + 1;

        match stmt.query_map(
            params![username, address, mailbox_path, limit, start],
            |row| {
                let html: String = row.get(17).unwrap();
                let text: String = row.get(18).unwrap();

                Ok(Message {
                    uid: row.get(0).unwrap(),
                    message_id: row.get(4).unwrap(),
                    subject: row.get(5).unwrap(),
                    from: row.get(6).unwrap(),
                    sender: row.get(7).unwrap(),
                    to: row.get(8).unwrap(),
                    cc: row.get(9).unwrap(),
                    bcc: row.get(10).unwrap(),
                    reply_to: row.get(11).unwrap(),
                    in_reply_to: row.get(12).unwrap(),
                    delivered_to: row.get(13).unwrap(),
                    date: row.get(14).unwrap(),
                    received: row.get(15).unwrap(),
                    flags: row.get(16).unwrap(),
                    html: BASE64_STANDARD.encode(html.as_bytes()),
                    text: BASE64_STANDARD.encode(text.as_bytes()),
                })
            },
        ) {
            Ok(messages) => {
                let mut messages_list: Vec<Message> = Vec::new();

                for message in messages {
                    match message {
                        Ok(message) => messages_list.push(message),
                        Err(_) => {
                            eprintln!("Message not present in local database");
                            continue;
                        }
                    }
                }

                return Ok(messages_list);
            }
            Err(e) => {
                eprintln!("Error getting message: {}", e);
                return Err(MyError::Sqlite(e));
            }
        };
    }
}
