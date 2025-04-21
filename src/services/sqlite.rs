use crate::constants::{DB_FILE, FOLDER_NAME};
use crate::objects::command::Command;
use rusqlite::{params, Connection};
use std::{fs, path::PathBuf};

fn get_connection() -> Result<Connection, String> {
    let mut config_dir = dirs_next::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.push(FOLDER_NAME);
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    config_dir.push(DB_FILE);

    let conn = Connection::open(config_dir).map_err(|e| e.to_string())?;

    Ok(conn)
}

fn upsert_commands_table() -> Result<(), String> {
    let conn = get_connection()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS commands (
            command_id INTEGER PRIMARY KEY,
            command TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

fn delete_all_commands() -> Result<(), String> {
    upsert_commands_table()?;
    let conn = get_connection()?;

    conn.execute("DELETE FROM commands", [])
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn upsert_command(command: &Command) -> Result<(), String> {
    upsert_commands_table()?;
    let conn = get_connection()?;

    conn.execute(
        "INSERT OR REPLACE INTO commands (command_id, command, description, created_at, updated_at)
       VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            command.command_id,
            command.command,
            command.description,
            command.created_at,
            command.updated_at
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn sync_commands(commands: &[Command]) -> Result<(), String> {
    upsert_commands_table()?;
    let mut conn = get_connection()?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    {
        let mut stmt = tx.prepare(
          "INSERT OR REPLACE INTO commands (command_id, command, description, created_at, updated_at)
           VALUES (?1, ?2, ?3, ?4, ?5)",
        ).map_err(|e| e.to_string())?;

        for cmd in commands {
            stmt.execute(params![
                cmd.command_id,
                cmd.command,
                cmd.description,
                cmd.created_at,
                cmd.updated_at
            ])
            .map_err(|e| e.to_string())?;
        }
    }

    if !commands.is_empty() {
        let ids: Vec<i32> = commands.iter().map(|c| c.command_id as i32).collect();
        let placeholders = ids
            .iter()
            .map(|_| "?".to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "DELETE FROM commands WHERE command_id NOT IN ({})",
            placeholders
        );

        let id_refs: Vec<&dyn rusqlite::ToSql> =
            ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();
        tx.execute(&sql, &*id_refs).map_err(|e| e.to_string())?;
    } else {
        tx.execute("DELETE FROM commands", [])
            .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}

fn read_all_commands() -> Result<Vec<Command>, String> {
    upsert_commands_table()?;
    let conn = get_connection()?;

    let mut stmt = conn
        .prepare("SELECT command_id, command, description, created_at, updated_at FROM commands")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Command {
                command_id: row.get(0)?,
                command: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();

    for row in rows {
        results.push(row.map_err(|e| e.to_string())?);
    }

    Ok(results)
}

pub fn search_commands(keywords: &[String]) -> Result<Vec<Command>, String> {
    upsert_commands_table()?;

    if keywords.is_empty() {
        return read_all_commands();
    }

    let conn = get_connection()?;

    let conditions: Vec<String> = keywords
        .iter()
        .map(|_| "(command LIKE ? OR description LIKE ?)".to_string())
        .collect();

    let where_clause = conditions.join(" AND ");

    let sql = format!(
        "SELECT command_id, command, description, created_at, updated_at FROM commands WHERE {}",
        where_clause
    );

    let mut sql_args: Vec<String> = vec![];

    for kw in keywords {
        let like = format!("%{}%", kw);
        sql_args.push(like.clone());
        sql_args.push(like);
    }

    let param_refs: Vec<&dyn rusqlite::ToSql> =
        sql_args.iter().map(|s| s as &dyn rusqlite::ToSql).collect();

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(&*param_refs, |row| {
            Ok(Command {
                command_id: row.get(0)?,
                command: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = vec![];

    for row in rows {
        results.push(row.map_err(|e| e.to_string())?);
    }

    Ok(results)
}
