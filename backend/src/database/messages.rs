use async_std::sync::{Arc, Mutex};
use async_std::task;
use base64::{prelude::BASE64_STANDARD, Engine};
use rusqlite::{params, types::Value, vtab, Connection};

use crate::database;
use crate::my_error::MyError;
use crate::types::database_request::{DatabaseRequest, MessageIdType, MessageReturnData};
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
            let err = MyError::Sqlite(
                e,
                String::from("Error starting transaction for inserting messages"),
            );
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
                let err =
                    MyError::FromUtf8(e, String::from("Error decoding text bytes for database"));
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
            Ok(_) => {}
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
                    let err =
                        MyError::Sqlite(e, String::from("Error inserting flag into database"));
                    err.log_error();

                    return Err(err);
                }
            }
        }
    }

    match tx.commit() {
        Ok(_) => {}
        Err(e) => {
            let err = MyError::Sqlite(
                e,
                String::from("Error committing transaction for inserting messages"),
            );
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

pub async fn get(
    conn: Arc<Mutex<Connection>>,
    request: DatabaseRequest,
) -> Result<Vec<Message>, MyError> {
    let locked_conn = conn.lock().await;

    let (query, highest_param) = construct_sql_query(&request);

    let mut list: Option<vtab::array::Array> = None;
    if request.id_rarray.is_some() {
        list = Some(std::rc::Rc::new(
            request
                .id_rarray
                .unwrap()
                .into_iter()
                .map(|id| Value::from(id))
                .collect::<Vec<Value>>(),
        ));
    }

    let mut limit: Option<u32> = None;
    if request.start.is_some() && request.end.is_some() {
        limit = Some(request.end.unwrap() - request.start.unwrap() + 1);
    }

    let username = &request.username.as_str();
    let address = &request.address.as_str();
    let mailbox_path = &request.mailbox_path.as_str();
    let list = &list.as_ref();

    let iter: Vec<&dyn rusqlite::types::ToSql> = vec![
        username,
        address,
        mailbox_path,
        list,
        &request.flag,
        &limit,
        &request.start,
    ];
    let iter = iter.get(0..highest_param).unwrap();

    let mut stmt = match locked_conn.prepare_cached(&query) {
        Ok(stmt) => stmt,
        Err(e) => {
            let err = MyError::Sqlite(e, String::from("Error preparing statement at messages"));
            err.log_error();

            return Err(err);
        }
    };

    match stmt.query_map(iter, |row| Ok(Message::from_row(row, &request.return_data))) {
        Ok(messages) => {
            let messages: Vec<Message> = messages
                .filter_map(|message| match message {
                    Ok(message) => Some(message),
                    Err(e) => {
                        let err =
                            MyError::Sqlite(e, String::from("Error getting message from database"));
                        err.log_error();

                        return None;
                    }
                })
                .collect();

            return Ok(messages);
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
    let database_request = DatabaseRequest {
        username: username.to_string(),
        address: address.to_string(),
        mailbox_path: mailbox_path.to_string(),
        return_data: MessageReturnData::Flags,
        id_type: MessageIdType::MessageUids,
        sorted: true,
        start: None,
        end: None,
        id_rarray: None,
        flag: None,
        not_flag: None,
    };

    let list = get(conn, database_request).await?;

    let mut flags: Vec<(u32, String)> = Vec::new();
    for message in list {
        for flag in message.flags {
            flags.push((message.message_uid, flag));
        }
    }

    return Ok(flags);
}

pub async fn get_flags_with_rarray(
    conn: Arc<Mutex<Connection>>,
    username: &str,
    address: &str,
    mailbox_path: &str,
    id_rarray: &Vec<u32>,
    id_type: MessageIdType,
) -> Result<Vec<(u32, String)>, MyError> {
    let database_request = DatabaseRequest {
        username: username.to_string(),
        address: address.to_string(),
        mailbox_path: mailbox_path.to_string(),
        return_data: MessageReturnData::Flags,
        id_type: id_type.clone(),
        sorted: match id_type {
            MessageIdType::MessageUids => true,
            MessageIdType::SequenceIds => false,
        },
        start: None,
        end: None,
        id_rarray: Some(id_rarray.clone()),
        flag: None,
        not_flag: None,
    };

    let messages = match get(conn, database_request).await {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let mut flags: Vec<(u32, String)> = Vec::new();
    for message in messages {
        for flag in message.flags {
            flags.push((message.message_uid, flag));
        }
    }

    return Ok(flags);
}

fn construct_sql_query(request: &DatabaseRequest) -> (String, usize) {
    let mut query = String::from("SELECT ");

    let mut highest_param = 3;

    let mut get_flags = false;
    match request.return_data {
        MessageReturnData::All => {
            query.push_str("* ");
        }
        MessageReturnData::AllWithFlags => {
            query.push_str("messages.*, flags.flag ");
            get_flags = true;
        }
        MessageReturnData::Flags => {
            query.push_str("messages.message_uid, flags.flag ");
            get_flags = true;
        }
        MessageReturnData::Uid => {
            query.push_str("messages.message_uid ");
        }
    }
    query.push_str("FROM messages ");

    if (request.flag.is_some() && request.not_flag.is_some()) || get_flags {
        if request.not_flag.is_some() && request.not_flag.unwrap() {
            query.push_str("LEFT JOIN flags ON messages.message_uid = flags.message_uid AND messages.c_username = flags.c_username AND messages.c_address = flags.c_address AND messages.m_path = flags.m_path ");
        } else {
            query.push_str("INNER JOIN flags ON messages.message_uid = flags.message_uid AND messages.c_username = flags.c_username AND messages.c_address = flags.c_address AND messages.m_path = flags.m_path ");
        }
    }

    query.push_str(
        "WHERE messages.c_username = ?1 AND messages.c_address = ?2 AND messages.m_path = ?3 ",
    );

    if request.id_rarray.is_some() {
        query.push_str("AND ");
        match request.id_type {
            MessageIdType::MessageUids => {
                query.push_str("messages.message_uid IN rarray(?4) ");
            }
            MessageIdType::SequenceIds => {
                query.push_str("messages.sequence_id IN rarray(?4) ");
            }
        }

        highest_param = 4;
    }

    if request.flag.is_some() && request.not_flag.is_some() {
        if !request.not_flag.unwrap() {
            query.push_str("AND flags.flag = ?5 ");
        } else {
            query.push_str(
                "AND messages.message_uid NOT IN (SELECT message_uid FROM flags WHERE flag = ?5) ",
            );
        }

        highest_param = 5;
    }

    if request.sorted {
        query.push_str("ORDER BY messages.received DESC ");
    }

    if request.start.is_some() && request.end.is_some() {
        query.push_str("LIMIT ?6 OFFSET ?7 ");

        highest_param = 7;
    }

    return (query, highest_param);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_sql_query_rarray_uids() {
        let request = DatabaseRequest {
            username: "username".to_string(),
            address: "address".to_string(),
            mailbox_path: "mailbox_path".to_string(),
            return_data: MessageReturnData::All,
            id_type: MessageIdType::MessageUids,
            sorted: false,
            start: None,
            end: None,
            id_rarray: Some(vec![1, 2, 3, 4, 5]),
            flag: None,
            not_flag: None,
        };

        let (query, highest_param) = construct_sql_query(&request);
        assert_eq!(query, "SELECT * FROM messages WHERE messages.c_username = ?1 AND messages.c_address = ?2 AND messages.m_path = ?3 AND messages.message_uid IN rarray(?4) ");
        assert_eq!(highest_param, 4);
    }

    #[test]
    fn construct_sql_query_rarray_seq() {
        let request = DatabaseRequest {
            username: "username".to_string(),
            address: "address".to_string(),
            mailbox_path: "mailbox_path".to_string(),
            return_data: MessageReturnData::All,
            id_type: MessageIdType::SequenceIds,
            sorted: false,
            start: None,
            end: None,
            id_rarray: Some(vec![1, 2, 3, 4, 5]),
            flag: None,
            not_flag: None,
        };

        let (query, highest_param) = construct_sql_query(&request);
        assert_eq!(query, "SELECT * FROM messages WHERE messages.c_username = ?1 AND messages.c_address = ?2 AND messages.m_path = ?3 AND messages.sequence_id IN rarray(?4) ");
        assert_eq!(highest_param, 4);
    }

    #[test]
    fn construct_sql_query_start_end() {
        let request = DatabaseRequest {
            username: "username".to_string(),
            address: "address".to_string(),
            mailbox_path: "mailbox_path".to_string(),
            return_data: MessageReturnData::All,
            id_type: MessageIdType::MessageUids,
            sorted: false,
            start: Some(1),
            end: Some(5),
            id_rarray: None,
            flag: None,
            not_flag: None,
        };

        let (query, highest_param) = construct_sql_query(&request);
        assert_eq!(query, "SELECT * FROM messages WHERE messages.c_username = ?1 AND messages.c_address = ?2 AND messages.m_path = ?3 LIMIT ?6 OFFSET ?7 ");
        assert_eq!(highest_param, 7);
    }

    #[test]
    fn construct_sql_query_start_end_sorted() {
        let request = DatabaseRequest {
            username: "username".to_string(),
            address: "address".to_string(),
            mailbox_path: "mailbox_path".to_string(),
            return_data: MessageReturnData::All,
            id_type: MessageIdType::MessageUids,
            sorted: true,
            start: Some(1),
            end: Some(5),
            id_rarray: None,
            flag: None,
            not_flag: None,
        };

        let (query, highest_param) = construct_sql_query(&request);
        assert_eq!(query, "SELECT * FROM messages WHERE messages.c_username = ?1 AND messages.c_address = ?2 AND messages.m_path = ?3 ORDER BY messages.received DESC LIMIT ?6 OFFSET ?7 ");
        assert_eq!(highest_param, 7);
    }

    #[test]
    fn construct_sql_query_with_flag() {
        let request = DatabaseRequest {
            username: "username".to_string(),
            address: "address".to_string(),
            mailbox_path: "mailbox_path".to_string(),
            return_data: MessageReturnData::Flags,
            id_type: MessageIdType::MessageUids,
            sorted: false,
            start: None,
            end: None,
            id_rarray: None,
            flag: Some("flag".to_string()),
            not_flag: Some(false),
        };

        let (query, highest_param) = construct_sql_query(&request);
        assert_eq!(query, "SELECT messages.message_uid, flags.flag FROM messages INNER JOIN flags ON messages.message_uid = flags.message_uid AND messages.c_username = flags.c_username AND messages.c_address = flags.c_address AND messages.m_path = flags.m_path WHERE messages.c_username = ?1 AND messages.c_address = ?2 AND messages.m_path = ?3 AND flags.flag = ?5 ");
        assert_eq!(highest_param, 5);
    }

    #[test]
    fn construct_sql_query_without_flag() {
        let request = DatabaseRequest {
            username: "username".to_string(),
            address: "address".to_string(),
            mailbox_path: "mailbox_path".to_string(),
            return_data: MessageReturnData::Flags,
            id_type: MessageIdType::MessageUids,
            sorted: false,
            start: None,
            end: None,
            id_rarray: None,
            flag: Some("flag".to_string()),
            not_flag: Some(true),
        };

        let (query, highest_param) = construct_sql_query(&request);
        assert_eq!(query, "SELECT messages.message_uid, flags.flag FROM messages LEFT JOIN flags ON messages.message_uid = flags.message_uid AND messages.c_username = flags.c_username AND messages.c_address = flags.c_address AND messages.m_path = flags.m_path WHERE messages.c_username = ?1 AND messages.c_address = ?2 AND messages.m_path = ?3 AND messages.message_uid NOT IN (SELECT message_uid FROM flags WHERE flag = ?5) ");
        assert_eq!(highest_param, 5);
    }

    #[test]
    fn construct_sql_query_with_flags() {
        let request = DatabaseRequest {
            username: "username".to_string(),
            address: "address".to_string(),
            mailbox_path: "mailbox_path".to_string(),
            return_data: MessageReturnData::Flags,
            id_type: MessageIdType::MessageUids,
            sorted: false,
            start: None,
            end: None,
            id_rarray: None,
            flag: None,
            not_flag: None,
        };

        let (query, highest_param) = construct_sql_query(&request);
        assert_eq!(query, "SELECT messages.message_uid, flags.flag FROM messages INNER JOIN flags ON messages.message_uid = flags.message_uid AND messages.c_username = flags.c_username AND messages.c_address = flags.c_address AND messages.m_path = flags.m_path WHERE messages.c_username = ?1 AND messages.c_address = ?2 AND messages.m_path = ?3 ");
        assert_eq!(highest_param, 3);
    }

    #[test]
    fn construct_sql_query_all_with_flags() {
        let request = DatabaseRequest {
            username: "username".to_string(),
            address: "address".to_string(),
            mailbox_path: "mailbox_path".to_string(),
            return_data: MessageReturnData::AllWithFlags,
            id_type: MessageIdType::MessageUids,
            sorted: false,
            start: None,
            end: None,
            id_rarray: None,
            flag: None,
            not_flag: None,
        };

        let (query, highest_param) = construct_sql_query(&request);
        assert_eq!(query, "SELECT messages.*, flags.flag FROM messages INNER JOIN flags ON messages.message_uid = flags.message_uid AND messages.c_username = flags.c_username AND messages.c_address = flags.c_address AND messages.m_path = flags.m_path WHERE messages.c_username = ?1 AND messages.c_address = ?2 AND messages.m_path = ?3 ");
        assert_eq!(highest_param, 3);
    }
}
