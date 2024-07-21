use crate::inbox_client::parse_message as parse_message;

use std::net::TcpStream;
use native_tls::{
    TlsConnector,
    TlsStream,
};
use imap;

pub struct SequenceSet {
  pub nr_messages: Option<usize>,
  pub start_end: Option<StartEnd>,
}

#[derive(Clone)]
pub struct StartEnd {
  pub start: usize,
  pub end: usize,
}

pub struct InboxClient {
  pub sessions: Vec<imap::Session<TlsStream<TcpStream>>>,
  pub addresses: Vec<String>,
}

impl InboxClient {
  pub fn new() -> InboxClient {
      InboxClient {
          sessions: Vec::new(),
          addresses: Vec::new(),
      }
  }

  pub fn connect(&mut self, address: &str, port: u16, username: &str, password: &str) -> Result<usize, String> {
      let tls = TlsConnector::builder().build().unwrap();
  
      match imap::connect((address, port), address, &tls) {
          Ok(c) => {
              match c.login(username, password) {
                  Ok(s) => {
                      
                      self.sessions.push(s);
                      self.addresses.push(String::from(username));

                      return Ok(self.sessions.len() - 1);
                  },
                  Err(e) => {
                      eprintln!("Error logging in: {:?}", e);
                      return Err(String::from("Error logging in"));
                  }
              }
          },
          Err(e) => {
              eprintln!("Error connecting to IMAP server: {}", e);
              return Err(String::from("Error connecting to IMAP server"));
          }
      };
  }

  pub fn logout(&mut self, session_id: usize) -> Result<(), String> {
      if session_id >= self.sessions.len() {
          return Err(String::from("Invalid session ID"));
      }

      let session = &mut self.sessions[session_id];

      match session.logout() {
          Ok(_) => {
              self.sessions.remove(session_id);
              self.addresses.remove(session_id);

              return Ok(());
          },
          Err(e) => {
              eprintln!("Error logging out: {:?}", e);
              return Err(String::from("Error logging out"));
          }
      }
  }

  pub fn get_messages(&mut self, session_id: usize, mailbox: String, sequence_set: SequenceSet) -> Result<String, String> {
      if session_id >= self.sessions.len() {
          return Err(String::from("Invalid session ID"));
      }

      let session = &mut self.sessions[session_id];

      session.select(mailbox).unwrap();

      let sequence_set_string: String = match sequence_set {
          SequenceSet { nr_messages: Some(nr_messages), start_end: None } => {
              format!("1:{}", nr_messages)
          },
          SequenceSet { nr_messages: None, start_end: Some(StartEnd { start, end }) } => {
              if start > end {
                  return Err(String::from("Start must be less than or equal to end"));
              }

              format!("{}:{}", start, end)
          },
          _ => return Err(String::from("Provide either nr_messages or start and end")),
      };

      let message_envelopes = session.fetch(sequence_set_string.clone(), "ENVELOPE").unwrap();
      let message_uids = session.fetch(sequence_set_string, "UID").unwrap();

      let mut response = String::from("{\"messages\": [");

      for (i, message) in message_envelopes.iter().enumerate() {
          let message_uid = match message_uids[i].uid {
              Some(uid) => uid,
              None => 0,
          };

          let message_string = parse_message::envelope_to_string(message, message_uid);

          response.push_str(&message_string);
          if i < message_envelopes.len() - 1 {
              response.push_str(",");
          }
      }

      response.push_str("]}");

      Ok(response)
  }

  pub fn get_message(&mut self, session_id: usize, mailbox: String, message_uid: u32) -> Result<String, String> {
      if session_id >= self.sessions.len() {
          return Err(String::from("Invalid session ID"));
      }

      let session = &mut self.sessions[session_id];

      session.select(mailbox).unwrap();

      let messages = session.uid_fetch(message_uid.to_string(), "BODY[TEXT]").unwrap();

      match messages.first() {
          Some(message) => {
              let message = parse_message::message_to_string(message, message_uid);

              return Ok(message);
          },
          None => return Err(String::from("Message not found")),
      };
  }

  pub fn get_all_mailboxes(&mut self, session_id: usize) -> Result<String, String> {
      if session_id >= self.sessions.len() {
          return Err(String::from("Invalid session ID"));
      }

      let session = &mut self.sessions[session_id];

      let mailboxes = session.list(Some(""), Some("*")).unwrap();

      let mut response = String::from("{\"mailboxes\": [");

      for (i, mailbox) in mailboxes.iter().enumerate() {
          response.push_str(&format!("\"{}\"", mailbox.name()));

          if i < mailboxes.len() - 1 {
              response.push_str(",");
          }
      }

      response.push_str("]}");

      Ok(response)
  }
}
