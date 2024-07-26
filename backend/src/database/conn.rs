use crate::inbox_client::parse_message::MessageBody;

use rusqlite::{params, Connection, OpenFlags, Result};

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
                id INTEGER PRIMARY KEY,
                username VARCHAR(500) NOT NULL UNIQUE,
                password VARCHAR(500) NOT NULL,
                address VARCHAR(500) NOT NULL,
                port INTEGER NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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
                id INTEGER PRIMARY KEY,
                connection_username VARCHAR(500) NOT NULL UNIQUE,
                path VARCHAR(500) NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(connection_username) REFERENCES connections(username) ON DELETE CASCADE
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
                id INTEGER PRIMARY KEY,
                connection_username VARCHAR(500) NOT NULL UNIQUE,
                mailbox_path VARCHAR(500) NOT NULL,
                uid INTEGER NOT NULL,
                message_id VARCHAR(500) NOT NULL,
                subject VARCHAR(500) NOT NULL,
                from VARCHAR(500) NOT NULL,
                sender VARCHAR(500) NULL,
                to VARCHAR(500) NOT NULL,
                cc VARCHAR(500) NULL,
                bcc VARCHAR(500) NULL,
                reply_to VARCHAR(500) NULL,
                in_reply_to VARCHAR(500) NULL,
                delivered_to VARCHAR(500) NULL,
                date TIMESTAMP NOT NULL,
                received TIMESTAMP NULL,
                html TEXT NULL,
                text TEXT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(connection_username) REFERENCES connections(username) ON DELETE CASCADE
                FOREIGN KEY(mailbox_path) REFERENCES mailboxes(path) ON DELETE CASCADE
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

    pub fn insert_message(&mut self, username: &str, mailbox: &str, message: &MessageBody) {
        match self.conn.execute(
            "INSERT INTO messages (
                connection_username,
                mailbox_path,
                uid,
                message_id,
                subject,
                from,
                sender,
                to,
                cc,
                bcc,
                reply_to,
                in_reply_to,
                delivered_to,
                date,
                received,
                html,
                text
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                username,
                mailbox,
                message.uid,
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
                message.html,
                message.text
            ],
        ) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error inserting message: {}", e);
            }
        }
    }

    pub fn get_message_with_uid(
        &mut self,
        username: &str,
        mailbox_path: &str,
        uid: &u32,
    ) -> Result<MessageBody, String> {
        let mut stmt = match self.conn.prepare(
            "SELECT * FROM messages WHERE connection_username = ?1 AND mailbox_path = ?2 AND uid = ?3",
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement: {}", e);
                return Err(String::from("Error preparing statement"));
            }
        };

        match stmt.query_row(params![username, mailbox_path, uid], |row| {
            Ok(MessageBody {
                connection_username: row.get(1).unwrap(),
                mailbox_path: row.get(2).unwrap(),
                uid: row.get(3).unwrap(),
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
                text: row.get(16).unwrap(),
                html: row.get(17).unwrap(),
            })
        }) {
            Ok(message) => {
                return Ok(message);
            }
            Err(e) => {
                eprintln!("Error getting message: {}", e);
                return Err(String::from("Error getting message"));
            }
        };
    }
}
