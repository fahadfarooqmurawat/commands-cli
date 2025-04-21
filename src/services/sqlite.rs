use rusqlite::{Connection, Result};

pub fn open_db() -> Result<Connection> {
    let conn = Connection::open("mycli_cache.db")?;
    Ok(conn)
}

pub fn create_commands_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS commands (
            id INTEGER PRIMARY KEY,
            command TEXT NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}
