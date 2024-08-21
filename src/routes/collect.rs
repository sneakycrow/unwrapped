use crate::{
    db::{self, spotify::upsert_playlogs, DBError},
    music::spotify::{RecentTrack, RecentTrackExt, SpotifyClient, SpotifyError},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use entity::{album, artist, play_log, track};
use sea_orm::{sqlx::types::chrono::DateTime, ActiveValue::NotSet, DatabaseConnection, Set};
use serde::Deserialize;
use tracing::{debug, error};

#[derive(Deserialize, Debug)]
pub struct Tokens {
    access_token: String,
    refresh_token: Option<String>,
}

struct Collection {
    recent_tracks: Option<Vec<RecentTrack>>,
    updated_token: Option<String>,
    db_artists: Option<Vec<artist::Model>>,
    db_albums: Option<Vec<album::Model>>,
    db_tracks: Option<Vec<track::Model>>,
}

impl Collection {
    fn new() -> Self {
        Self {
            recent_tracks: None,
            updated_token: None,
            db_artists: None,
            db_albums: None,
            db_tracks: None,
        }
    }

    /// An internal function for collecting recent tracks from Spotify
    /// In addition to getting the tracks, this function also handles refreshing the access token if it is invalid
    async fn collect_recent_tracks(
        &mut self,
        access_token: String,
        refresh_token: Option<String>,
    ) -> Result<&mut Self, SpotifyError> {
        // Generate a client for interacting with Spotify
        let client = SpotifyClient::new(access_token).set_refresh_token(refresh_token);
        // Fetch the recent tracks from Spotify
        match client.get_recent_tracks().await {
            Ok(recent_tracks) => {
                self.recent_tracks = Some(recent_tracks.items.unwrap_or_default());
                Ok(self)
            }
            Err(spotify_err) => {
                match spotify_err.status {
                    // If the error is an invalid token error, try to get a new access token
                    401 => {
                        debug!("Invalid Token error, Attempting to get a new access token");
                        // First get the new access token
                        let new_token = client.refresh_access_token().await?;
                        // Update the client with the new access token and try to get the recent tracks again
                        let recent_tracks = client
                            .set_access_token(new_token.access_token.clone())
                            .get_recent_tracks()
                            .await?;
                        // Return the recent tracks and the new access token
                        self.recent_tracks = Some(recent_tracks.items.unwrap_or_default());
                        self.updated_token = Some(new_token.access_token);
                        Ok(self)
                    }
                    _ => Err(spotify_err),
                }
            }
        }
    }

    /// Upsert the artists from the recent tracks into the database
    async fn upsert_artists(&mut self, conn: &DatabaseConnection) -> Result<&mut Self, DBError> {
        // Parse the artists and albums from the recent tracks and save them
        // We store Artists, Albums, and Tracks separately, then use those ID's to craft a "PlayLog" entry
        // Top to bottom, artists -> albums -> tracks -> playlog
        debug!("Parsing artists from recent tracks");
        let artist_models = self
            .recent_tracks
            .as_ref()
            .expect("No recent tracks found, cannot upsert artists")
            .artists()
            .into_iter()
            .map(|artist| artist.model())
            .collect();
        // Upsert all the artists, returning the artists with their ID's
        debug!("Upserting artists into database");
        let db_artists = db::spotify::upsert_artists(artist_models, conn)
            .await
            .expect("Error upserting artists");

        self.db_artists = Some(db_artists);
        Ok(self)
    }
    /// Upsert the albums from the recent tracks into the database
    async fn upsert_albums(&mut self, conn: &DatabaseConnection) -> Result<&mut Self, DBError> {
        // Next, convert the recent track albums to their models, using our databases artist IDs and save the albums/album artists
        debug!("Parsing albums from recent tracks");
        let raw_albums_with_artists: Vec<(album::ActiveModel, Vec<artist::Model>)> = self
            .recent_tracks
            .as_ref()
            .expect("No recent tracks found, cannot upsert albums")
            // Get the raw spotify albums
            .albums()
            .into_iter()
            // Convert to Album and AlbumArtist models using the artist ID's
            .map(|album| {
                // Get the album active model
                let db_album = album.model();
                // Find the artists for the album
                let album_artists: Vec<artist::Model> = album
                    // Iterate over all the album artists from spotify
                    .artists
                    .iter()
                    // Find the relative db artists based on the recent tracks album artists
                    .map(|alb_artist| {
                        self.db_artists
                            .as_ref()
                            .expect("No artists found, cannot parse artist for upserting album")
                            .iter()
                            // Find the artist by their name
                            .find(|db_artist| alb_artist.name == db_artist.name)
                            .expect("Artist not found")
                            .to_owned()
                    })
                    .collect();

                (db_album, album_artists)
            })
            .collect();
        // Upsert the albums with their artists, returning the albums with their ID's
        debug!("Upserting albums into database");
        let db_albums_with_artists =
            db::spotify::upsert_albums_with_artists(raw_albums_with_artists, conn)
                .await
                .expect("Error upserting albums");

        self.db_albums = Some(db_albums_with_artists);
        Ok(self)
    }
    /// Upsert the tracks from the recent tracks into the database
    async fn upsert_tracks(&mut self, conn: &DatabaseConnection) -> Result<&mut Self, DBError> {
        // Each track should reference an artist and an album, and then use the album to also create an album track
        let raw_tracks_with_albums: Vec<(track::ActiveModel, album::Model)> = self
            .recent_tracks
            .as_ref()
            .expect("No recent tracks found, cannot upsert tracks")
            .into_iter()
            .map(|recent_track| {
                // Get the track active model
                let db_track = recent_track.track.model();
                // Find the album
                let db_album = self
                    .db_albums
                    .as_ref()
                    .expect("No albums found, cannot parse album for upserting track")
                    .iter()
                    .find(|album| album.title == recent_track.track.album.name)
                    .expect("Album not found")
                    .to_owned();
                (db_track, db_album)
            })
            .collect();
        // Upsert the tracks with their albums, returning the tracks with their ID's
        let db_tracks_with_albums =
            db::spotify::upsert_tracks_with_albums(raw_tracks_with_albums, conn)
                .await
                .expect("Error upserting tracks");

        self.db_tracks = Some(db_tracks_with_albums);
        Ok(self)
    }
    /// Upsert the playlogs from the recent tracks into the database
    async fn upsert_playlogs(&mut self, conn: &DatabaseConnection) -> Result<&mut Self, DBError> {
        // Finally, create the playlogs from the recent tracks
        let raw_playlogs: Vec<play_log::ActiveModel> = self
            .recent_tracks
            .as_ref()
            .expect("No recent tracks found, cannot upsert playlogs")
            .into_iter()
            .map(|recent_track| {
                // Get the track
                let db_track = self
                    .db_tracks
                    .as_ref()
                    .expect("No tracks found, cannot parse track for upserting playlog")
                    .iter()
                    .find(|track| track.title == recent_track.track.name)
                    .expect("Track not found")
                    .to_owned();
                // Parse the played_at into a DateTime timestamp. Should be up to seconds
                let timestamp = DateTime::parse_from_rfc3339(&recent_track.played_at)
                    .expect("Error parsing played_at from track")
                    .naive_utc();
                // Create the playlog
                play_log::ActiveModel {
                    id: NotSet,
                    track_id: Set(db_track.id),
                    played_at: Set(timestamp),
                }
            })
            .collect();

        db::spotify::upsert_playlogs(raw_playlogs, conn)
            .await
            .expect("Error upserting playlogs");

        Ok(self)
    }
}

/// Collect goes to each of the configured providers, collects the relative data, and saves it to the DB
pub async fn route(
    State(state): State<crate::routes::AppState>,
    tokens: Query<Tokens>,
) -> Result<(), (StatusCode, String)> {
    // Collect the tokens from the query
    let access_token = tokens.access_token.to_owned();
    let refresh_token = tokens.refresh_token.to_owned();
    // Initialize the collection
    Collection::new()
        // Collect tracks from spotify
        .collect_recent_tracks(access_token, refresh_token)
        .await
        .map_err(|spotify_err| {
            error!("Error collecting recent tracks: {:?}", spotify_err);
            (StatusCode::INTERNAL_SERVER_ERROR, spotify_err.message)
        })?
        // Save artists
        .upsert_artists(&state.connection)
        .await
        .map_err(|db_err| {
            error!("Error upserting artists: {:?}", db_err);
            (StatusCode::INTERNAL_SERVER_ERROR, db_err.to_string())
        })?
        // Save albums
        .upsert_albums(&state.connection)
        .await
        .map_err(|db_err| {
            error!("Error upserting albums: {:?}", db_err);
            (StatusCode::INTERNAL_SERVER_ERROR, db_err.to_string())
        })?
        // Save tracks
        .upsert_tracks(&state.connection)
        .await
        .map_err(|db_err| {
            error!("Error upserting tracks: {:?}", db_err);
            (StatusCode::INTERNAL_SERVER_ERROR, db_err.to_string())
        })?
        // Finally, save the playlogs
        .upsert_playlogs(&state.connection)
        .await
        .map_err(|db_err| {
            error!("Error upserting playlogs: {:?}", db_err);
            (StatusCode::INTERNAL_SERVER_ERROR, db_err.to_string())
        })?;
    // Return Ok if everything was successful
    debug!("Successfully collected and upserted recent tracks");
    Ok(())
}
