use std::collections::HashMap;
use url_escape::decode;

use crate::my_error::MyError;

pub fn get_usize(uri_param: Option<&String>) -> Result<Option<usize>, MyError> {
    match uri_param {
        Some(param) => match Some(param.parse::<usize>()) {
            Some(Ok(p)) => Ok(Some(p)),
            Some(Err(e)) => Err(MyError::ParseInt(
                e,
                format!("Error parsing usize {}", param.to_string()),
            )),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

pub fn get_u16(uri_param: Option<&String>) -> Result<Option<u16>, MyError> {
    match uri_param {
        Some(param) => match Some(param.parse::<u16>()) {
            Some(Ok(p)) => Ok(Some(p)),
            Some(Err(e)) => Err(MyError::ParseInt(
                e,
                format!("Error parsing u16 {}", param.to_string()),
            )),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

pub fn get_u32(uri_param: Option<&String>) -> Result<Option<u32>, MyError> {
    match uri_param {
        Some(param) => match Some(param.parse::<u32>()) {
            Some(Ok(p)) => Ok(Some(p)),
            Some(Err(e)) => Err(MyError::ParseInt(
                e,
                format!("Error parsing u32 {}", param.to_string()),
            )),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

pub fn get_bool(uri_param: Option<&String>) -> Result<Option<bool>, MyError> {
    match uri_param {
        Some(param) => match Some(param.parse::<bool>()) {
            Some(Ok(p)) => Ok(Some(p)),
            Some(Err(e)) => Err(MyError::ParseBool(
                e,
                format!("Error parsing bool {}", param),
            )),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

pub fn parse_params(uri: String) -> HashMap<String, String> {
    let uri_parts: Vec<&str> = uri.split("&").collect();

    let mut result: HashMap<String, String> = HashMap::new();

    uri_parts.iter().for_each(|part| {
        let parts: Vec<&str> = part.split("=").collect();

        if parts.len() != 2 {
            return;
        }

        let key = parts[0].to_owned();
        let value = decode(parts[1]).to_string();

        result.insert(key, value);
    });

    return result;
}
