use async_std::sync::{Arc, Mutex};
use rusqlite::{params, types::Value, vtab, Connection};
use base64::{prelude::BASE64_STANDARD, Engine};
use async_std::task;

use crate::database;
use crate::my_error::MyError;
use crate::types::message::Message;

pub async fn insert(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    messages: &Vec<Message>,
  ) -> Result<(), MyError> {
    let mut locked_conn = conn.lock().await;
    
    let tx = match locked_conn.transaction() {
        Ok(tx) => tx,
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error starting transaction for inserting messages"));
            err.log_error();
  
            return Err(err);
        }
    };

    for message in messages {    
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
        
        match tx.execute(
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
html,
text
) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
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
                html,
                text
            ],
        ) {
            Ok(_) => {},
            Err(e) => {
                
  
                let err = MyError::Sqlite(e, String::from("Error inserting message into database"));
                err.log_error();
  
                return Err(err);
            }
        };

        for flag in &message.flags {
            match tx.execute(
                "INSERT OR IGNORE INTO flags (
message_uid,
c_username,
c_address,
m_path,
flag
) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![message.message_uid, username, address, mailbox_path, flag],
            ) {
                Ok(_) => {}
                Err(e) => {
                    let err = MyError::Sqlite(e, String::from("Error inserting flag into database"));
                    err.log_error();
    
                    return Err(err);
                }
            }
        }
    }

    match tx.commit() {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error committing transaction for inserting messages"));
            err.log_error();

            return Err(err);
        }
    }

    drop(locked_conn);
    task::spawn(async move {
        match database::backup(conn).await {
            Ok(_) => {}
            Err(e) => e.log_error(),
        }
    });

    return Ok(());
}

pub async fn get_with_rarray(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    id_rarray: &Vec<u32>,
    message_uids: bool,
) -> Result<Vec<Message>, MyError> {
    let list: vtab::array::Array = std::rc::Rc::new(
        id_rarray
            .into_iter()
            .map(|uid| Value::from(*uid))
            .collect::<Vec<Value>>(),
    );

    let conn_2 = Arc::clone(&conn);
    let flags_data = match get_flags_with_rarray(conn_2, username, address, mailbox_path, &list, true).await {
        Ok(flags) => flags,
        Err(e) => return Err(e),
    };
 
    let locked_conn = conn.lock().await;

    let query: &str;
    if message_uids {
        query = "SELECT * FROM messages WHERE message_uid IN rarray(?1) AND c_username = ?2 AND c_address = ?3 AND m_path = ?4";
    } else {
        query = "SELECT * FROM messages WHERE sequence_id IN rarray(?1) AND c_username = ?2 AND c_address = ?3 AND m_path = ?4";
    }


    let mut stmt = match locked_conn.prepare_cached(query) {
            Ok(stmt) => stmt,
            Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error preparing statement at messages with uids"));
            err.log_error();

            return Err(err);
        }
    };

    match stmt.query_map(params![list, username, address, mailbox_path], |row| {
        Ok(Message::from_row(row))
    }) {
        Ok(messages_data) => {
            let messages_list: Vec<Message> = messages_data.filter_map(|data| {
                match data {
                    Ok(mut message) => {
                        let mut message_flags: Vec<String> = flags_data
                            .iter()
                            .filter(|f| f.0 == message.message_uid)
                            .map(|f| f.1.to_string())
                            .collect();

                        message.flags.append(&mut message_flags);
                        return Some(message);
                    },
                    Err(e) => {
                        let err = MyError::Sqlite(e, String::from("Error getting message from database"));
                        err.log_error();
    
                        return None;
                    }
                }
            }).collect();

            return Ok(messages_list);
        }
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error getting message from database"));
            err.log_error();

            return Err(err);
        }
    };
}
  
pub async fn get_sorted(
  conn: Arc<Mutex<Connection>>,
  username: &str,
  address: &str,
  mailbox_path: &str,
  start: u32,
  end: u32,
) -> Result<Vec<Message>, MyError> {
  let limit = end - start + 1;

  let locked_conn = conn.lock().await;
  
  let mut stmt = match locked_conn.prepare_cached(
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

pub async fn get_flags(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
) -> Result<Vec<(u32, String)>, MyError> {
    let locked_conn = conn.lock().await;
    
    let mut stmt = match locked_conn.prepare_cached(
            "SELECT message_uid, flags FROM flags WHERE c_username = ?1 AND c_address = ?2 AND m_path = ?3",
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                
                let err = MyError::Sqlite(e, String::from("Error preparing statement at messages"));
                err.log_error();
  
                return Err(err);
            }
        };
  
    match stmt.query_map(
        params![username, address, mailbox_path],
        |row| {
            let message_uid: u32 = row.get(0).unwrap();
            let flag: String = row.get(1).unwrap();

            return Ok((message_uid, flag));
        },
    ) {
        Ok(flags_data) => {
            let flags_data: Vec<(u32, String)> = flags_data.filter_map(|data| {
                match data {
                    Ok(d) => Some(d),
                    Err(e) => {
                        let err = MyError::Sqlite(e, String::from("Error getting message from database"));
                        err.log_error();
    
                        return None;
                    }
                }
            }).collect();

            return Ok(flags_data);
        }
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error getting message from database"));
  
            err.log_error();
            return Err(err);
        }
    };
}

pub async fn get_flags_with_rarray(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    list: &vtab::array::Array,
    message_uid: bool,
) -> Result<Vec<(u32, String)>, MyError> {
    let locked_conn = conn.lock().await;

    let query: &str;
    if message_uid {
        query = "SELECT * FROM flags WHERE message_uid IN rarray(?1) AND c_username = ?2 AND c_address = ?3 AND m_path = ?4";
    } else {
        query = "SELECT * FROM flags WHERE sequence_id IN rarray(?1) AND c_username = ?2 AND c_address = ?3 AND m_path = ?4";
    }

    let mut stmt = match locked_conn.prepare_cached(query) {
            Ok(stmt) => stmt,
            Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error preparing statement at messages with uids"));
            err.log_error();

            return Err(err);
        }
    };

    let flags_data: Vec<(u32, String)> = match stmt.query_map(
        params![list, username, address, mailbox_path],
        |row| {
            let message_uid: u32 = row.get(0).unwrap();
            let flag: String = row.get(1).unwrap();

            return Ok((message_uid, flag));
        },
    ) {
        Ok(flags_data) => {
            flags_data.filter_map(|data| {
                match data {
                    Ok(d) => Some(d),
                    Err(e) => {
                        let err = MyError::Sqlite(e, String::from("Error getting message from database"));
                        err.log_error();

                        return None;
                    }
                }
            }).collect()
        }
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error getting message from database"));

            err.log_error();
            return Err(err);
        }
    };

    return Ok(flags_data);
}
