use chrono::{DateTime, FixedOffset};
use regex::Regex;

pub fn rfc2822(time_str: Option<&String>) -> DateTime<FixedOffset> {
    let time_re =
        Regex::new(r"(\w{1,3}, \d{1,2} \w{1,3} \d{4} \d{2}:\d{2}:\d{2} ([+-]\d{4})?(\w{3})?)")
            .unwrap();
    let binding = String::from("");

    let date = match time_re.captures(time_str.unwrap_or(&binding)) {
        Some(c) => c.get(1).unwrap().as_str(),
        None => {
            eprintln!("Error: Could not parse date");
            "Thu, 1 Jan 1970 00:00:00 +0000"
        }
    };

    let date = match DateTime::parse_from_rfc2822(&date) {
        Ok(date) => date,
        Err(e) => {
            eprintln!("Error: {}", e);
            DateTime::parse_from_rfc2822("Thu, 1 Jan 1970 00:00:00 +0000").unwrap()
        }
    };

    return date;
}
