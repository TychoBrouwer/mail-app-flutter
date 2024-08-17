#[derive(Debug, Clone)]
pub enum MessageIdType {
    MessageUids,
    SequenceIds,
}

#[derive(Debug, Clone)]
pub enum MessageReturnData {
    All,
    AllWithFlags,
    Flags,
}

pub struct DatabaseRequest {
    pub username: String,
    pub address: String,
    pub mailbox_path: String,
    pub return_data: MessageReturnData,
    pub id_type: MessageIdType,
    pub sorted: bool,
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub id_rarray: Option<Vec<u32>>,
    pub flag: Option<String>,
    pub not_flag: Option<bool>,
}
