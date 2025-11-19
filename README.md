# Clippy

Simple Rust clipboard tracker and viewer.

- `checker.exe`: runs in background, stores clipboard entries.
- `clippy.exe`: prints clipboard entries from the database.

Flags for `clippy.exe`:
- `--amount` / `-a`: print last X entries (default all)
- `--hide_time`: hide time column
- `--hide_date`: hide date column
- `--clear`: clear all entries

Database stored in local AppData folder (`Clippy/clipboard.db` on Windows).

Built with:
- Rust
- arboard
- rusqlite
- clap
- chrono
- colored
- dirs

Probably sucks, trying to get better at Rust, don't use seriously.
(no support outside of windows)
