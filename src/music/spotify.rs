use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyError {
    status: u16,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    name: String,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    images: Vec<AlbumImage>,
    name: String,
    release_date: String,
    external_urls: ExternalUrls,
    artists: Vec<Artist>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumImage {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentTrack {
    track: Track,
    played_at: String,
}

/// The primary client for interacting with the Spotify API
pub struct SpotifyClient {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentTracksResponse {
    pub items: Option<Vec<RecentTrack>>,
    pub error: Option<SpotifyError>,
}

impl SpotifyClient {
    pub fn new(access_token: String) -> Self {
        SpotifyClient {
            token: access_token,
        }
    }

    pub async fn get_recent_tracks(&self) -> Result<RecentTracksResponse, SpotifyError> {
        const ENDPOINT: &'static str = "https://api.spotify.com/v1/me/player/recently-played";
        let tracks: RecentTracksResponse = surf::get(ENDPOINT)
            .header("Authorization", format!("Bearer {}", self.token))
            .recv_json()
            .await
            .map_err(|err| {
                error!("Failed to get recent tracks from Spotify: {:?}", err);
                SpotifyError {
                    status: 500,
                    message: "Internal error requesting recent tracks from Spotify".to_string(),
                }
            })?;
        // If we received an error from Spotify, it should still parse as an Ok result
        // but we still represent this as an error, just a recoverable parsed error
        if let Some(error) = tracks.error {
            error!("Spotify API returned an error: {:?}", error);
            return Err(error);
        }
        Ok(tracks)
    }
}
