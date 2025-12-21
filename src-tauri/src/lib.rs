use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use tauri_plugin_sql::{Migration, MigrationKind};

#[derive(Debug, Serialize, Deserialize)]
struct SpotifyListen {
    pub ts: String,
    pub ms_played: i64,
    pub master_metadata_track_name: Option<String>,
    pub master_metadata_album_artist_name: Option<String>,
    pub spotify_track_uri: Option<String>,
}

// Command that takes the path of an extended history JSON
// and converts them into the SpotifyListen struct
#[tauri::command]
fn import_extended_history(path: String) -> Result<String, String> {
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let listens: Vec<SpotifyListen> = serde_json::from_reader(reader).map_err(|e| e.to_string())?;

    Ok(format!("Successfully read {} songs from the file!", listens.len()))
}

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
