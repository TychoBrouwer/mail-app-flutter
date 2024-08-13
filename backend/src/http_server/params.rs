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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_params_test() {
        let uri = String::from("key1=value1&key2=value2");

        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert(String::from("key1"), String::from("value1"));
        expected.insert(String::from("key2"), String::from("value2"));

        assert_eq!(parse_params(uri), expected);
    }

    #[test]
    fn parse_params_test_empty() {
        let uri = String::from("");

        let expected: HashMap<String, String> = HashMap::new();

        assert_eq!(parse_params(uri), expected);
    }

    #[test]
    fn parse_params_test_invalid() {
        let uri = String::from("key1=value1&key2");

        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert(String::from("key1"), String::from("value1"));

        assert_eq!(parse_params(uri), expected);
    }

    #[test]
    fn get_usize_test() {
        let param = String::from("1");

        assert_eq!(get_usize(Some(&param)).unwrap(), Some(1));
    }

    #[test]
    fn get_usize_test_empty() {
        let param = None;

        assert_eq!(get_usize(param).unwrap(), None);
    }

    #[test]
    fn get_usize_test_invalid() {
        let param = String::from("invalid");

        assert!(get_usize(Some(&param)).is_err());
    }

    #[test]
    fn get_u16_test() {
        let param = String::from("1");

        assert_eq!(get_u16(Some(&param)).unwrap(), Some(1));
    }

    #[test]
    fn get_u16_test_empty() {
        let param = None;

        assert_eq!(get_u16(param).unwrap(), None);
    }

    #[test]
    fn get_u16_test_invalid() {
        let param = String::from("invalid");

        assert!(get_u16(Some(&param)).is_err());
    }

    #[test]
    fn get_u32_test() {
        let param = String::from("1");

        assert_eq!(get_u32(Some(&param)).unwrap(), Some(1));
    }

    #[test]
    fn get_u32_test_empty() {
        let param = None;

        assert_eq!(get_u32(param).unwrap(), None);
    }

    #[test]
    fn get_u32_test_invalid() {
        let param = String::from("invalid");

        assert!(get_u32(Some(&param)).is_err());
    }

    #[test]
    fn get_bool_test() {
        let param = String::from("true");

        assert_eq!(get_bool(Some(&param)).unwrap(), Some(true));
    }

    #[test]
    fn get_bool_test_2() {
        let param = String::from("True");

        assert!(get_bool(Some(&param)).is_err());
    }

    #[test]
    fn get_bool_test_3() {
        let param = String::from("0");

        assert!(get_bool(Some(&param)).is_err());
    }

    #[test]
    fn get_bool_test_empty() {
        let param = None;

        assert_eq!(get_bool(param).unwrap(), None);
    }

    #[test]
    fn get_bool_test_invalid() {
        let param = String::from("invalid");

        assert!(get_bool(Some(&param)).is_err());
    }
}
