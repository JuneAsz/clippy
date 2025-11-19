#![windows_subsystem = "windows"] // invisible window

use arboard::Clipboard;
use clippy::*;
use rusqlite::Connection;
use std::{fs::OpenOptions, io::Write, time};

/// Log errors to a file instead of printing, since console is hidden
fn log_error(message: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("clippy_errors.log")
    {
        let _ = writeln!(file, "{}", message);
    }
}

fn init_clipboard_safe() -> Option<Clipboard> {
    match Clipboard::new() {
        Ok(clipboard) => Some(clipboard),
        Err(e) => {
            log_error(&format!("Failed to initialize clipboard: {}", e));
            None
        }
    }
}

fn run_loop_safe(clipboard: &mut Clipboard, connection: &Connection) {
    let mut last_clipboard = String::new();

    loop {
        let text = match clipboard.get_text() {
            Ok(text) => text,
            Err(e) => {
                log_error(&format!("Failed to get clipboard text: {}", e));
                std::thread::sleep(time::Duration::from_secs(1));

                continue; // skip this iteration, don't exit
            }
        };

        if text != last_clipboard {
            last_clipboard = text.clone();

            let now = chrono::Local::now();
            let current_date = now.date_naive().to_string();
            let current_time = now.time().format("%H:%M:%S").to_string();

            let entry = ClipboardEntry {
                date: current_date,
                time: current_time,
                content: text.clone(),
            };

            if let Err(e) = store_entry(connection, entry) {
                log_error(&format!("Failed to insert clipboard entry: {}", e));
            }
        }

        std::thread::sleep(time::Duration::from_secs(1));
    }
}

fn main() {
    let mut clipboard = match init_clipboard_safe() {
        Some(c) => c,
        None => return, // failed to initialize, stop program
    };

    let connection = match init_database() {
        Ok(c) => c,
        Err(e) => {
            log_error(&format!("Failed to initialize database: {}", e));
            return;
        }
    };

    run_loop_safe(&mut clipboard, &connection);
}
