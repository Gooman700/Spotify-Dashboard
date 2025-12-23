mod models;
mod commands;
use commands::import_extended_history::import_extended_history;
use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_stream_table",
            sql: "CREATE TABLE IF NOT EXISTS streams (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts TEXT,
                track_name TEXT,
                artist_name TEXT,
                ms_played INTEGER,
                track_uri TEXT,
                UNIQUE(ts, track_uri)
            );",
            kind: MigrationKind::Up
        }
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:dashboard.db", migrations)
                .build()
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![import_extended_history])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
