use async_std::sync::{Arc, Mutex};
use rusqlite::{params, types::Value, vtab, Connection};

use crate::my_error::MyError;
use crate::types::message::Message;

pub async fn get_with_uids(
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

pub async fn get_sorted(
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
