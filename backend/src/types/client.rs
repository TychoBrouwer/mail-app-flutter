pub struct Client<'a> {
    pub address: &'a str,
    pub port: u16,
    pub username: &'a str,
    pub password: &'a str,
}
