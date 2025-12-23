use std::fs;
use std::io::BufReader;
use std::path::Path;
use crate::models::spotify_listen::SpotifyListen;

fn process_single_audio_file(path: &Path) -> Result<usize, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    
    let listens: Vec<SpotifyListen> = serde_json::from_reader(reader)
        .map_err(|e| e.to_string())?;

    Ok(listens.len())
}

#[tauri::command]
pub async fn import_extended_history(folder_path: String) -> Result<String, String> {
    let entries = fs::read_dir(folder_path).map_err(|e| e.to_string())?;
    let mut total_songs = 0;

    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        if path.is_file() && file_name.contains("Audio") && file_name.ends_with(".json") {
            total_songs += process_single_audio_file(&path)?;
        }
    }
    Ok(format!("Imported {} total songs!", total_songs))
}