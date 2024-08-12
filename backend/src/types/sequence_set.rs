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
                if reversed {
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
                    let mut begin = exists - end + 1;
                    let mut last = exists - start + 1;

                    if exists < end + 1 {
                        begin = 1;
                    }

                    if exists < start + 1 {
                        last = 1;
                    }

                    if exists < end + 1 && exists < start + 1 {
                        begin = u32::MAX;
                        last = u32::MAX;
                    }

                    format!("{}:{}", begin, last)
                } else {
                    format!("{}:{}", start, end)
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
