pub mod client;

use entity::{album, artist};
use sea_orm::{prelude::Date, ActiveValue, NotSet};
use serde::{Deserialize, Serialize};
use surf::Body;

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
