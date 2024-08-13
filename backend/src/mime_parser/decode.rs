pub fn to_u8(string: Option<&[u8]>) -> String {
    match string {
        Some(s) => match std::str::from_utf8(s) {
            Ok(s) => String::from(s),
            Err(_) => String::from(""),
        },
        None => String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8() {
        assert_eq!(to_u8(Some(b"test")), String::from("test"));
        assert_eq!(to_u8(None), String::from(""));
    }
}
