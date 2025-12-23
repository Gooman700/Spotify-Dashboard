use serde::{Deserialize, Serialize};

// Struct containing the useful information for a listen
#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyListen {
    pub ts: String,
    
    #[serde(rename = "ms_played")] 
    pub duration: i64,

    #[serde(rename = "master_metadata_track_name")]
    pub track: Option<String>,

    #[serde(rename = "master_metadata_album_artist_name")]
    pub artist: Option<String>,

    #[serde(rename = "spotify_track_uri")]
    pub uri: Option<String>,
}