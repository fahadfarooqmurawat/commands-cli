use crate::objects::command::Command;
use rusqlite::{params, Connection, Result};

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

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS commands (
            command_id INTEGER PRIMARY KEY,
            command TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn upsert_command(conn: &Connection, cmd: &Command) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO commands (command_id, command, description, created_at, updated_at)
       VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            cmd.command_id,
            cmd.command,
            cmd.description,
            cmd.created_at,
            cmd.updated_at
        ],
    )?;
    Ok(())
}

pub fn upsert_commands(conn: &mut Connection, commands: &[Command]) -> Result<()> {
    let tx = conn.transaction()?;

    {
        let mut stmt = tx.prepare(
          "INSERT OR REPLACE INTO commands (command_id, command, description, created_at, updated_at)
           VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;

        for cmd in commands {
            stmt.execute(params![
                cmd.command_id,
                cmd.command,
                cmd.description,
                cmd.created_at,
                cmd.updated_at
            ])?;
        }
    }

    tx.commit()?;
    Ok(())
}

pub fn get_all_commands(conn: &Connection) -> Result<Vec<Command>> {
    let mut stmt = conn
        .prepare("SELECT command_id, command, description, created_at, updated_at FROM commands")?;
    let rows = stmt.query_map([], |row| {
        Ok(Command {
            command_id: row.get(0)?,
            command: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

pub fn search_commands(conn: &Connection, keywords: &[String]) -> Result<Vec<Command>> {
    if keywords.is_empty() {
        return Ok(vec![]);
    }

    // Construct SQL WHERE clause with OR conditions
    let conditions: Vec<String> = keywords
        .iter()
        .map(|_| "(command LIKE ? OR description LIKE ?)".to_string())
        .collect();
    let where_clause = conditions.join(" OR ");

    // Prepare SQL query
    let sql = format!(
        "SELECT command_id, command, description, created_at, updated_at FROM commands WHERE {}",
        where_clause
    );

    // Prepare the LIKE parameters
    let mut sql_args: Vec<String> = vec![];
    for kw in keywords {
        let like = format!("%{}%", kw);
        sql_args.push(like.clone());
        sql_args.push(like);
    }

    // Convert to Vec<&dyn ToSql>
    let param_refs: Vec<&dyn rusqlite::ToSql> =
        sql_args.iter().map(|s| s as &dyn rusqlite::ToSql).collect();

    // Execute query
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(&*param_refs, |row| {
        Ok(Command {
            command_id: row.get(0)?,
            command: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    // Collect results
    let mut results = vec![];
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}
