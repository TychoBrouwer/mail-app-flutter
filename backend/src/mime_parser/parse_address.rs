use async_imap::imap_proto::Address;

use crate::mime_parser::decode;

pub fn to_string(address: &Option<Vec<Address>>) -> String {
    match address {
        Some(a) => {
            let mut result = String::from("[");

            for (i, address) in a.iter().enumerate() {
                result.push_str("{");
                result.push_str(&format!(
                    "\"name\": \"{}\",",
                    decode::to_u8(address.name.as_deref())
                ));
                result.push_str(&format!(
                    "\"mailbox\": \"{}\",",
                    decode::to_u8(address.mailbox.as_deref())
                ));
                result.push_str(&format!(
                    "\"host\": \"{}\"",
                    decode::to_u8(address.host.as_deref())
                ));
                result.push_str("}");

                if i < a.len() - 1 {
                    result.push_str(",");
                }
            }

            result.push_str("]");

            return result;
        }
        None => return String::from("[]"),
    }
}
