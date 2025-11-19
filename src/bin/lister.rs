// src\bin\lister.rs

use clap::Parser;
use clippy::*;

#[derive(Parser)]
#[command(version, about = "Clippy", long_about = None)]
struct Args {
    /// Print last X entries (default all)
    #[arg(short, long, default_value_t = 0)]
    amount: usize,

    /// Hide time in output
    #[arg(long, default_value_t = false)]
    hide_time: bool,

    /// Hide date in output
    #[arg(long, default_value_t = false)]
    hide_date: bool,

    /// Clear all entries
    #[arg(long, default_value_t = false)]
    clear: bool,
}

fn main() {
    let connection = match init_database() {
        Ok(connection) => connection,
        Err(e) => {
            eprintln!("Error trying to initialize connection: {}", e);
            return;
        }
    };

    let args = Args::parse();

    let amount = args.amount;
    let hide_time = args.hide_time;
    let hide_date = args.hide_date;
    let clear = args.clear;

    if clear {
        clear_database(&connection);
    }

    print_entries_with_flags_and_amount(&connection, hide_time, hide_date, amount);
}
