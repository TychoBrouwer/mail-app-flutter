use crate::inbox_client::parse_message::Message;

use rusqlite::{params, Connection, OpenFlags, Result};
use base64::{prelude::BASE64_STANDARD, Engine};

pub struct DBConnection {
    conn: Connection,
}

impl DBConnection {
    pub fn new(database_path: &str) -> Result<DBConnection> {
        let conn = match Connection::open_with_flags(
            database_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        ) {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Error opening database: {}", e);
                return Err(e);
            }
        };

        return Ok(DBConnection { conn });
    }

    pub fn initialise(&mut self) -> Result<()> {
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

                return Err(e);
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

                return Err(e);
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

                return Err(e);
            }
        }

        return Ok(());
    }

    pub fn insert_connection(&mut self, username: &str, password: &str, address: &str, port: u16) {
        match self.conn.execute(
            "INSERT OR IGNORE INTO connections (
                username,
                password,
                address,
                port
            ) VALUES (?1, ?2, ?3, ?4)",
            params![username, password, address, port],
        ) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error inserting connection: {}", e);
            }
        }
    }

    pub fn insert_mailbox(&mut self, username: &str, address: &str, mailbox_path: &str) {
        match self.conn.execute(
            "INSERT OR IGNORE INTO mailboxes (
                c_username,
                c_address,
                path
            ) VALUES (?1, ?2, ?3)",
            params![username, address, mailbox_path],
        ) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error inserting mailbox: {}", e);
            }
        }
    }

    pub fn insert_message(
        &mut self,
        username: &str,
        address: &str,
        mailbox_path: &str,
        message: &Message,
    ) -> Result<(), String> {
        let html = match String::from_utf8(BASE64_STANDARD.decode(message.html.as_str()).unwrap()) {
            Ok(html) => html,
            Err(e) => {
                eprintln!("Error decoding HTML: {}", e);

                return Err(String::from("Error decoding HTML"));
            }
        };

        let text = match String::from_utf8(BASE64_STANDARD.decode(message.text.as_str()).unwrap()) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Error decoding text: {}", e);

                return Err(String::from("Error decoding text"));
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
                html,
                text
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
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
                html,
                text
            ],
        ) {
            Ok(_) => Ok({}),
            Err(e) => {
                eprintln!("Error inserting message: {}", e);

                return Err(String::from("Error inserting message"));
            }
        }
    }

    pub fn get_mailboxes(&mut self, username: &str, address: &str) -> Result<Vec<String>, String> {
        let mut stmt = match self
            .conn
            .prepare("SELECT * FROM mailboxes WHERE c_username = ?1 AND c_address = ?2")
        {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement at mailboxes: {}", e);
                return Err(String::from("Error preparing statement at mailboxes"));
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
                return Err(String::from("Error getting mailboxes"));
            }
        }

        return Ok(mailboxes);
    }

    pub fn get_message_with_uid(
        &mut self,
        username: &str,
        address: &str,
        mailbox_path: &str,
        uid: &u32,
    ) -> Result<Message, String> {
        let mut stmt = match self.conn.prepare(
            "SELECT * FROM messages WHERE c_username = ?1 AND c_address = ?2 AND m_path = ?3 AND uid = ?4",
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement: {}", e);
                return Err(String::from("Error preparing statement"));
            }
        };


        match stmt.query_row(params![username, address, mailbox_path, uid], |row| {
            let html: String = row.get(16).unwrap();
            let text: String = row.get(17).unwrap();

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
                html: BASE64_STANDARD.encode(html.as_bytes()),
                text: BASE64_STANDARD.encode(text.as_bytes()),
            })
        }) {
            Ok(message) => {
                return Ok(message);
            }
            Err(e) => {
                eprintln!("Error getting message: {}", e);
                return Err(String::from("Error getting message from local database"));
            }
        };
    }
}
