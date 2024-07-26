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
                username VARCHAR(500) NOT NULL,
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
                connection_id INTEGER NOT NULL,
                path VARCHAR(500) NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(connection_id) REFERENCES connections(id) ON DELETE CASCADE
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
                uid INTEGER PRIMARY KEY,
                connection_id INTEGER NOT NULL,
                mailbox_id INTEGER NOT NULL,
                message_id VARCHAR(500) NOT NULL,
                subject VARCHAR(500) NOT NULL,
                from VARCHAR(500) NOT NULL,
                sender VARCHAR(500) NOT NULL,
                to VARCHAR(500) NOT NULL,
                cc VARCHAR(500) NOT NULL,
                bcc VARCHAR(500) NOT NULL,
                reply_to VARCHAR(500) NOT NULL,
                in_reply_to VARCHAR(500) NOT NULL,
                delivered_to VARCHAR(500) NOT NULL,
                date TIMESTAMP NOT NULL,
                received TIMESTAMP NOT NULL,
                html TEXT NOT NULL,
                text TEXT NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(connection_id) REFERENCES connections(id) ON DELETE CASCADE,
                FOREIGN KEY(mailbox_id) REFERENCES mailboxes(id) ON DELETE CASCADE
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

    fn get_connection_id(&mut self, username: &str, address: &str) -> Result<u32, String> {
        let mut stmt = match self
            .conn
            .prepare("SELECT * FROM connections WHERE username = ?1 AND address = ?2")
        {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement at connection id: {}", e);
                return Err(String::from("Error preparing statement at connection id"));
            }
        };

        let connection_id = match stmt.query_row(params![username, address], |row| row.get(0)) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error retreiving connection id: {}", e);

                return Err(String::from("Error retreiving connection id"));
            }
        };

        return Ok(connection_id);
    }

    fn get_mailbox_id(&mut self, connection_id: &u32, mailbox_path: &str) -> Result<u32, String> {
        let mut stmt = match self
            .conn
            .prepare("SELECT * FROM mailboxes WHERE connection_id = ?1 AND path = ?2")
        {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement at mailbox id: {}", e);
                return Err(String::from("Error preparing statement at mailbox id"));
            }
        };

        let mailbox_id =
            match stmt.query_row(params![connection_id, mailbox_path], |row| row.get(0)) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error retreiving mailbox id: {}", e);

                    return Err(String::from("Error retreiving mailbox id"));
                }
            };

        return Ok(mailbox_id);
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
        let connection_id = match self.get_connection_id(username, address) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error getting connection id: {}", e);
                return;
            }
        };

        match self.conn.execute(
            "INSERT OR IGNORE INTO mailboxes (
                connection_id,
                path
            ) VALUES (?1, ?2)",
            params![mailbox_path, connection_id],
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
        message: &MessageBody,
    ) -> Result<(), String> {
        let connection_id = match self.get_connection_id(username, address) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        let mailbox_id = match self.get_mailbox_id(&connection_id, mailbox_path) {
            Ok(result) => result,
            Err(e) => {
                // Create mailbox database row
                return Err(e);
            }
        };

        match self.conn.execute(
            "INSERT OR IGNORE INTO messages (
                uid,
                connection_id,
                mailbox_id,
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
                connection_id,
                mailbox_id,
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
            Ok(_) => Ok({}),
            Err(e) => {
                eprintln!("Error inserting message: {}", e);

                return Err(String::from("Error inserting message"));
            }
        }
    }

    pub fn get_mailboxes(&mut self, username: &str, address: &str) -> Result<Vec<String>, String> {
        let connection_id = match self.get_connection_id(username, address) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error getting connection id: {}", e);
                return Err(String::from("Error getting connection id"));
            }
        };

        let mut stmt = match self
            .conn
            .prepare("SELECT * FROM mailboxes WHERE connection_id = ?1")
        {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Error preparing statement at mailboxes: {}", e);
                return Err(String::from("Error preparing statement at mailboxes"));
            }
        };

        let mut mailboxes: Vec<String> = Vec::new();

        match stmt.query_map(params![connection_id], |row| row.get(2)) {
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
                uid: row.get(0).unwrap(),
                message_id: row.get(3).unwrap(),
                subject: row.get(4).unwrap(),
                from: row.get(5).unwrap(),
                sender: row.get(6).unwrap(),
                to: row.get(7).unwrap(),
                cc: row.get(8).unwrap(),
                bcc: row.get(9).unwrap(),
                reply_to: row.get(10).unwrap(),
                in_reply_to: row.get(11).unwrap(),
                delivered_to: row.get(12).unwrap(),
                date: row.get(13).unwrap(),
                received: row.get(14).unwrap(),
                html: row.get(15).unwrap(),
                text: row.get(16).unwrap(),
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
