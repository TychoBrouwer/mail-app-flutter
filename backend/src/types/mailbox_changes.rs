pub struct ChangedSeqIdData {
    pub message_uid: u32,
    pub sequence_id_new: u32,
}

pub struct MailboxChanges {
    pub new: Vec<u32>,
    pub changed: Vec<u32>,
    pub changed_seq: Vec<ChangedSeqIdData>,
    pub removed: Vec<u32>,
}

impl MailboxChanges {
    pub fn new() -> MailboxChanges {
        return MailboxChanges {
            new: vec![],
            changed: vec![],
            changed_seq: vec![],
            removed: vec![],
        };
    }
}
