use base64::prelude::*;
use entity::{album, artist};
use sea_orm::{prelude::Date, ActiveValue, NotSet};
use serde::{Deserialize, Serialize};
use surf::{http::mime, Body, Url};
use tracing::{debug, error};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyError {
    pub status: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub name: String,
    pub album: Album,
    external_urls: ExternalUrls,
}

impl Into<entity::track::Entity> for Track {
    fn into(self) -> entity::prelude::Track {
        entity::prelude::Track {}
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artist {
    pub name: String,
    external_urls: ExternalUrls,
}

impl Artist {
    pub fn model(&self) -> artist::ActiveModel {
        artist::ActiveModel {
            id: NotSet,
            name: ActiveValue::set(self.name.clone()),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    images: Vec<AlbumImage>,
    pub name: String,
    release_date: String,
    album_type: String,
    external_urls: ExternalUrls,
    pub artists: Vec<Artist>,
}

impl Album {
    pub fn model(&self) -> album::ActiveModel {
        let release_date = Date::parse_from_str(&self.release_date, "%Y-%m-%d")
            .expect("Failed to parse release date");
        album::ActiveModel {
            id: NotSet,
            title: ActiveValue::set(self.name.clone()),
            release_date: ActiveValue::set(release_date),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumImage {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentTrack {
    pub track: Track,
    pub played_at: String,
}

impl Track {
    pub fn model(&self) -> entity::track::ActiveModel {
        entity::track::ActiveModel {
            id: NotSet,
            title: ActiveValue::set(self.name.clone()),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}

pub trait RecentTrackExt {
    fn artists(&self) -> Vec<Artist>;
    fn albums(&self) -> Vec<Album>;
}

impl RecentTrackExt for Vec<RecentTrack> {
    /// Gets all the artists from the recent tracks
    fn artists(&self) -> Vec<Artist> {
        self.iter()
            .map(|track| track.track.album.artists.clone())
            .flatten()
            .collect()
    }
    /// Gets all the albums from the recent tracks
    fn albums(&self) -> Vec<Album> {
        self.iter().map(|track| track.track.album.clone()).collect()
    }
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

#[derive(Serialize)]
struct SpotifyTokenRequest {
    code: String,
    redirect_uri: String,
    grant_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpotifyTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

impl Into<Body> for SpotifyTokenRequest {
    fn into(self) -> Body {
        Body::from_form(&self).expect("Failed to convert SpotifyTokenRequest to Body")
    }
}

/// The primary client for interacting with the Spotify API
pub struct SpotifyClient {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub client_id: String,
    pub client_secret: String,
    pub scopes: String,
    pub redirect_uri: String,
}

impl SpotifyClient {
    /// Create a new SpotifyClient with credentials from environment variables
    fn from_env() -> Self {
        // Get the values from environment variables
        let client_id = std::env::var("SPOTIFY_ID").expect("SPOTIFY_CLIENT_ID not set");
        let client_secret = std::env::var("SPOTIFY_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");
        let scopes = std::env::var("SPOTIFY_SCOPES").expect("SPOTIFY_SCOPES not set");
        let redirect_uri =
            std::env::var("SPOTIFY_REDIRECT_URI").expect("SPOTIFY_REDIRECT_URI not set");
        Self {
            access_token: None,
            refresh_token: None,
            client_id,
            client_secret,
            scopes,
            redirect_uri,
        }
    }
    /// Generate an authorization url for the user to login
    pub fn get_authorize_url() -> Url {
        const BASE_URL: &'static str = "https://accounts.spotify.com/authorize?";
        let creds = Self::from_env();
        Url::parse_with_params(
            BASE_URL,
            &[
                ("response_type", "code"),
                ("client_id", &creds.client_id),
                ("scope", &creds.scopes),
                ("redirect_uri", &creds.redirect_uri),
            ],
        )
        .expect("Failed to construct Spotify OAuth URL")
    }
    /// Get the access token from the authorization code
    pub async fn get_tokens(&mut self, code: String) -> Result<&mut Self, SpotifyError> {
        const BASE_URL: &'static str = "https://accounts.spotify.com/api/token";
        let auth_header =
            BASE64_STANDARD.encode(format!("{}:{}", self.client_id, self.client_secret));
        let res: SpotifyTokenResponse = surf::post(BASE_URL)
            .header("Authorization", format!("Basic {}", auth_header))
            .content_type(mime::FORM)
            .body(SpotifyTokenRequest {
                code,
                redirect_uri: self.redirect_uri.clone(),
                grant_type: "authorization_code".to_string(),
            })
            .recv_json()
            .await
            .map_err(|e| error!("Failed to request access token from Spotify: {}", e))
            .unwrap();

        Ok(self
            .set_access_token(res.access_token)
            .set_refresh_token(res.refresh_token))
    }
    /// Create a new SpotifyClient
    pub fn new() -> Self {
        Self::from_env()
    }
    /// Set the refresh token for the client
    pub fn set_refresh_token(&mut self, refresh_token: String) -> &mut Self {
        self.refresh_token = Some(refresh_token);
        self
    }
    /// Set the access token for the client
    pub fn set_access_token(&mut self, access_token: String) -> &mut Self {
        self.access_token = Some(access_token);
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
        let access_token = self.access_token.as_ref().unwrap();
        const ENDPOINT: &'static str = "https://api.spotify.com/v1/me/player/recently-played";
        let tracks: RecentTracksResponse = surf::get(ENDPOINT)
            .header("Authorization", format!("Bearer {}", access_token))
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
