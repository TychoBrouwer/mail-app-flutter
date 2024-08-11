#[derive(Debug, Clone)]
pub struct Client {
    pub address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}
