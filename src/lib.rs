// src\lib.rs
use colored::*;
use dirs;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

pub struct ClipboardEntry {
    pub date: String,
    pub time: String,
    pub content: String,
}

pub fn init_database() -> Result<Connection, rusqlite::Error> {
    // Use local app data folder on Windows
    let db_folder: PathBuf = match dirs::data_local_dir() {
        Some(mut path) => {
            path.push("Clippy");
            path
        }
        None => PathBuf::from("Clippy"), // fallback
    };

    // Create folder if it doesn't exist
    if let Err(e) = fs::create_dir_all(&db_folder) {
        eprintln!("Failed to create database folder {:?}: {}", db_folder, e);
        return Err(rusqlite::Error::InvalidPath(db_folder));
    }

    let db_path = db_folder.join("clipboard.db");

    let connection = match Connection::open(&db_path) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to open database {:?}: {}", db_path, e);
            return Err(e);
        }
    };

    if let Err(e) = connection.execute(
        "CREATE TABLE IF NOT EXISTS clipboard (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            time TEXT NOT NULL,
            content TEXT NOT NULL
        )",
        (),
    ) {
        eprintln!("Failed to create table in {:?}: {}", db_path, e);
        return Err(e);
    }

    Ok(connection)
}

pub fn store_entry(connection: &Connection, entry: ClipboardEntry) -> Result<(), rusqlite::Error> {
    let result = connection.execute(
        "INSERT INTO clipboard (
    date, time, content) VALUES (?1, ?2, ?3)",
        (&entry.date, &entry.time, &entry.content),
    );

    match result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error inserting entry into DB! {}", e);
            return Err(e);
        }
    }

    Ok(())
}

pub fn print_entries_with_flags_and_amount(
    connection: &rusqlite::Connection,
    hide_time: bool,
    hide_date: bool,
    amount: usize,
) {
    let mut prepared_statement = match connection
        .prepare("SELECT id, date, time, content FROM clipboard ORDER BY id DESC")
    {
        Ok(statement) => statement,
        Err(database_error) => {
            eprintln!("Failed to prepare statement: {}", database_error);
            return;
        }
    };

    let rows_iterator = match prepared_statement.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    }) {
        Ok(iterator) => iterator,
        Err(execution_error) => {
            eprintln!("Failed to execute query: {}", execution_error);
            return;
        }
    };

    let mut printed_count = 0;

    for row_result in rows_iterator {
        if amount > 0 && printed_count >= amount {
            break;
        }

        match row_result {
            Ok((entry_id, entry_date, entry_time, entry_content)) => {
                let mut output = String::new();

                output.push_str(&format!("[{}] ", entry_id.to_string().cyan()));

                if !hide_date {
                    output.push_str(&format!("{} ", entry_date.green()));
                }

                if !hide_time {
                    output.push_str(&format!("{} ", entry_time.dimmed()));
                }

                output.push_str(&entry_content.yellow().bold());

                println!("{}", output);

                printed_count += 1;
            }
            Err(row_error) => {
                eprintln!("Failed to read row: {}", row_error);
                break;
            }
        }
    }
}

pub fn clear_database(connection: &Connection) {
    let result = connection.execute("DELETE FROM clipboard", ());
    match result {
        Ok(affected_rows) => println!("Deleted {} rows", affected_rows),
        Err(e) => eprintln!("Failed to delete rows: {}", e),
    }
}
