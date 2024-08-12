use base64::prelude::*;
use serde::{Deserialize, Serialize};
use surf::http::mime;
use tracing::{debug, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyError {
    pub status: u16,
    pub message: String,
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
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentTracksResponse {
    pub items: Option<Vec<RecentTrack>>,
    pub error: Option<SpotifyError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: u32,
}

impl SpotifyClient {
    /// Create a new SpotifyClient with an access token
    pub fn new(access_token: String) -> Self {
        SpotifyClient {
            access_token,
            refresh_token: None,
        }
    }
    /// Set the refresh token for the client
    pub fn set_refresh_token(mut self, refresh_token: Option<String>) -> Self {
        self.refresh_token = refresh_token;
        self
    }
    /// Set the access token for the client
    pub fn set_access_token(mut self, access_token: String) -> Self {
        self.access_token = access_token;
        self
    }
    /// Get a new access token using the refresh token
    pub async fn refresh_access_token(&self) -> Result<RefreshTokenResponse, SpotifyError> {
        // If a refresh_token does not exist, we cannot get a new access token
        let refresh_token = match &self.refresh_token {
            Some(token) => token,
            None => {
                return Err(SpotifyError {
                    status: 400,
                    message: "No refresh token provided".to_string(),
                })
            }
        };
        // Get new token from Spotify
        let new_token = Self::request_access_token(refresh_token.clone())
            .await
            .map_err(|err| {
                error!("Failed to fetch new access token from Spotify {:?}", err);
                SpotifyError {
                    status: 500,
                    message: "Internal error requesting access token from Spotify".to_string(),
                }
            })?;
        // Return the new access token and refresh token
        Ok(new_token)
    }
    /// Fetch the recent tracks from Spotify
    pub async fn get_recent_tracks(&self) -> Result<RecentTracksResponse, SpotifyError> {
        debug!(
            "Fetching recent tracks from Spotify using access token {}",
            self.access_token
        );
        const ENDPOINT: &'static str = "https://api.spotify.com/v1/me/player/recently-played";
        let tracks: RecentTracksResponse = surf::get(ENDPOINT)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .recv_json()
            .await
            .map_err(|err| {
                error!("Failed to fetch json from spotify {:?}", err);
                SpotifyError {
                    status: 500,
                    message: "Internal error requesting recent tracks from Spotify".to_string(),
                }
            })?;
        // The request can successfully parse the JSON response, but the response itself can contain
        // an error from Spotify's API. If an error is present, return the error
        if let Some(error) = tracks.error {
            return Err(error);
        }
        Ok(tracks)
    }
    /// Send request to Spotify to refresh the access token
    pub(crate) async fn request_access_token(
        refresh_token: String,
    ) -> Result<RefreshTokenResponse, SpotifyError> {
        const ENDPOINT: &'static str = "https://accounts.spotify.com/api/token";
        let client_id = std::env::var("SPOTIFY_ID").map_err(|_| SpotifyError {
            status: 500,
            message: "Missing Spotify Client ID".to_string(),
        })?;
        let client_secret = std::env::var("SPOTIFY_SECRET").map_err(|_| SpotifyError {
            status: 500,
            message: "Missing Spotify Client Secret".to_string(),
        })?;
        let auth = BASE64_STANDARD.encode(format!("{}:{}", client_id, client_secret));
        let body = format!("grant_type=refresh_token&refresh_token={}", refresh_token);
        let token: RefreshTokenResponse = surf::post(ENDPOINT)
            .header("Authorization", format!("Basic {}", auth))
            .content_type(mime::FORM)
            .body(body)
            .recv_json()
            .await
            .map_err(|err| {
                error!("Failed to fetch json from spotify {:?}", err);
                SpotifyError {
                    status: 500,
                    message: "Internal error requesting access token from Spotify".to_string(),
                }
            })?;
        debug!(
            "Successfully fetched new access token from Spotify {}",
            token.access_token
        );
        Ok(token)
    }
}
