use std::fs;
use std::io::BufReader;
use std::path::Path;
use sqlx::{sqlite::SqlitePool};
use tauri::Manager;
use crate::models::spotify_listen::SpotifyListen;

async fn process_single_audio_file(path: &Path, pool: &SqlitePool) -> Result<usize, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    
    let listens: Vec<SpotifyListen> = serde_json::from_reader(reader)
        .map_err(|e| e.to_string())?;

    // Start the transaction. Using transaction so that only one expensive database write is needed.
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let mut added = 0;
    for listen in listens {
        // Skip entries that don't have a track name and don't have at least 30s listen time.
        if let Some(track_name) = listen.track {
            if listen.duration >= 30000 {
                sqlx::query(
                    "INSERT OR IGNORE INTO streams (ts, track_name, artist_name, ms_played, track_uri) 
                    VALUES (?, ?, ?, ?, ?)"
                )
                .bind(&listen.ts)
                .bind(&track_name)
                .bind(&listen.artist)
                .bind(&listen.duration)
                .bind(&listen.uri)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
                
                added += 1;
            }
        }
    }

    // Commit the transaction
    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(added)
}

#[tauri::command]
pub async fn import_extended_history(
    folder_path: String, 
    app_handle: tauri::AppHandle // Used to resolve the database path
) -> Result<String, String> {
    let app_dir = app_handle.path().app_config_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("dashboard.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());

    let pool = SqlitePool::connect(&db_url).await.map_err(|e| e.to_string())?;

    let entries = fs::read_dir(folder_path).map_err(|e| e.to_string())?;
    let mut total_songs = 0;

    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        if path.is_file() && file_name.contains("Audio") && file_name.ends_with(".json") {
            total_songs += process_single_audio_file(&path, &pool).await?;
        }
    }

    Ok(format!("Successfully imported {} songs to your database!", total_songs))
}