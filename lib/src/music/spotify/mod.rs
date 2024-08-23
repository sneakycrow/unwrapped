pub mod client;
pub mod vendor;

use serde::{Deserialize, Serialize};
use surf::Body;

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyError {
    pub status: u16,
    pub message: String,
}

pub trait RecentTrackExt {
    fn artists(&self) -> Vec<vendor::Artist>;
    fn albums(&self) -> Vec<vendor::Album>;
}

impl RecentTrackExt for Vec<vendor::RecentTrack> {
    /// Gets all the artists from the recent tracks
    fn artists(&self) -> Vec<vendor::Artist> {
        self.iter()
            .map(|track| track.track.album.artists.clone())
            .flatten()
            .collect()
    }
    /// Gets all the albums from the recent tracks
    fn albums(&self) -> Vec<vendor::Album> {
        self.iter().map(|track| track.track.album.clone()).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentTracksResponse {
    pub items: Option<Vec<vendor::RecentTrack>>,
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
