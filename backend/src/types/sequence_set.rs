use crate::my_error::MyError;

#[derive(Debug)]
pub struct StartEnd {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
pub struct SequenceSet {
    pub nr_messages: Option<u32>,
    pub start_end: Option<StartEnd>,
    pub idx: Option<Vec<u32>>,
}

impl SequenceSet {
    pub fn to_string(&self, exists: u32, reversed: bool) -> Result<String, MyError> {
        let sequence_set_string: String = match self {
            SequenceSet {
                nr_messages: Some(nr_messages),
                start_end: None,
                idx: None,
            } => {
                if nr_messages == &u32::MAX {
                    format!("1:*")
                } else if reversed {
                    let begin = exists - nr_messages + 1;
                    format!("{}:{}", begin, exists)
                } else {
                    format!("1:{}", nr_messages)
                }
            }
            SequenceSet {
                nr_messages: None,
                start_end: Some(StartEnd { start, end }),
                idx: None,
            } => {
                if start > end {
                    let err = MyError::String(
                        String::from("Start must be less than or equal to end"),
                        String::from("Error converting SequenceSet to string representation"),
                    );
                    err.log_error();

                    return Err(err);
                }

                if reversed {
                    let mut last = exists - start + 1;

                    if *end == u32::MAX {
                        format!("1:{}", last)
                    } else {
                        let mut begin = exists - end + 1;

                        if exists < end + 1 {
                            begin = 1;
                        }

                        if exists < start + 1 {
                            last = 1;
                        }

                        if exists < end + 1 && exists < start + 1 {
                            begin = u32::MAX - 1;
                            last = u32::MAX - 1;
                        }

                        if last == u32::MAX {
                            format!("{}:*", begin)
                        } else {
                            format!("{}:{}", begin, last)
                        }
                    }
                } else {
                    if *end == u32::MAX {
                        format!("{}:*", start)
                    } else {
                        format!("{}:{}", start, end)
                    }
                }
            }
            SequenceSet {
                nr_messages: None,
                start_end: None,
                idx: Some(idxs),
            } => {
                let mut result = String::new();

                for (i, idx) in idxs.iter().enumerate() {
                    if reversed {
                        result.push_str(&((exists - idx + 1).to_string()));
                    } else {
                        result.push_str(&idx.to_string());
                    }

                    if i < idxs.len() - 1 {
                        result.push_str(",");
                    }
                }

                result
            }
            _ => {
                if reversed {
                    format!("{}:*", exists)
                } else {
                    String::from("1:*")
                }
            }
        };

        return Ok(sequence_set_string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sequence_set(
        nr_messages: Option<u32>,
        start: Option<u32>,
        end: Option<u32>,
        idx: Option<Vec<u32>>,
    ) -> SequenceSet {
        SequenceSet {
            nr_messages,
            start_end: match start {
                Some(start) => Some(StartEnd {
                    start,
                    end: end.unwrap(),
                }),
                None => None,
            },
            idx,
        }
    }

    #[test]
    fn nr_messages() {
        let sequence_set = get_sequence_set(Some(5), None, None, None);
        let sequence_set_string = sequence_set.to_string(5, false).unwrap();
        assert_eq!(sequence_set_string, "1:5");
    }

    #[test]
    fn nr_messages_reversed() {
        let sequence_set = get_sequence_set(Some(4), None, None, None);
        let sequence_set_string = sequence_set.to_string(5, true).unwrap();
        assert_eq!(sequence_set_string, "2:5");
    }

    #[test]
    fn nr_messages_max() {
        let sequence_set = get_sequence_set(Some(u32::MAX), None, None, None);
        let sequence_set_string = sequence_set.to_string(5, false).unwrap();
        assert_eq!(sequence_set_string, "1:*");
    }

    #[test]
    fn start_end() {
        let sequence_set = get_sequence_set(None, Some(2), Some(5), None);
        let sequence_set_string = sequence_set.to_string(5, false).unwrap();
        assert_eq!(sequence_set_string, "2:5");
    }

    #[test]
    fn start_end_reversed() {
        let sequence_set = get_sequence_set(None, Some(2), Some(5), None);
        let sequence_set_string = sequence_set.to_string(5, true).unwrap();
        assert_eq!(sequence_set_string, "1:4");
    }

    #[test]
    fn start_end_max() {
        let sequence_set = get_sequence_set(None, Some(2), Some(u32::MAX), None);
        let sequence_set_string = sequence_set.to_string(5, false).unwrap();
        assert_eq!(sequence_set_string, "2:*");
    }

    #[test]
    fn start_end_max_2() {
        let sequence_set = get_sequence_set(None, Some(u32::MAX - 1), Some(u32::MAX), None);
        let sequence_set_string = sequence_set.to_string(5, false).unwrap();
        assert_eq!(sequence_set_string, format!("{}:*", u32::MAX - 1));
    }

    #[test]
    fn start_end_max_reversed() {
        let sequence_set = get_sequence_set(None, Some(2), Some(u32::MAX), None);
        let sequence_set_string = sequence_set.to_string(10, true).unwrap();
        assert_eq!(sequence_set_string, "1:9");
    }

    #[test]
    fn idx() {
        let sequence_set = get_sequence_set(None, None, None, Some(vec![2, 3, 5]));
        let sequence_set_string = sequence_set.to_string(5, false).unwrap();
        assert_eq!(sequence_set_string, "2,3,5");
    }

    #[test]
    fn idx_reversed() {
        let sequence_set = get_sequence_set(None, None, None, Some(vec![2, 3, 5]));
        let sequence_set_string = sequence_set.to_string(5, true).unwrap();
        assert_eq!(sequence_set_string, "4,3,1");
    }

    #[test]
    fn idx_reversed_2() {
        let sequence_set = get_sequence_set(None, None, None, Some(vec![2, 3, 5, 7, 8, 10]));
        let sequence_set_string = sequence_set.to_string(10, true).unwrap();
        assert_eq!(sequence_set_string, "9,8,6,4,3,1");
    }
}
