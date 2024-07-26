use rusqlite::{params,Connecion,Result};

pub fn initialise(database_path: &str) -> Result<Connection> {
    let conn = Connection::open_with_flags(database_path);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY,
            message TEXT NOT NULL
        )",
        params![],
    )?;
}