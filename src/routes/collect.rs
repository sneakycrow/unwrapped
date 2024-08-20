use crate::{
    db,
    music::spotify::{RecentTrack, RecentTrackExt, SpotifyClient, SpotifyError},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use entity::{album, artist, track};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

#[derive(Serialize)]
pub struct CollectionResponse {
    updated_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Tokens {
    access_token: String,
    refresh_token: Option<String>,
}

/// Collect goes to each of the configured providers, collects the relative data, and saves it to the DB
pub async fn route(
    State(_state): State<crate::routes::AppState>,
    tokens: Query<Tokens>,
) -> Result<Json<CollectionResponse>, (StatusCode, String)> {
    // Collect the tokens from the query
    let access_token = tokens.access_token.to_owned();
    let refresh_token = tokens.refresh_token.to_owned();
    // Get the recent tracks from Spotify
    let (recent_tracks, updated_token) = collect_recent_tracks(access_token, refresh_token)
        .await
        .map_err(|spotify_err| {
        error!("Error collecting recent tracks: {:?}", spotify_err);
        (StatusCode::INTERNAL_SERVER_ERROR, spotify_err.message)
    })?;
    debug!("Successfully collected recent track data, parsing artists and albums");
    // Parse the artists and albums from the recent tracks and save them
    // We store Artists, Albums, and Tracks separately, then use those ID's to craft a "PlayLog" entry
    // Top to bottom, artists -> albums -> tracks -> playlog
    debug!("Parsing artists from recent tracks");
    let artist_models = recent_tracks
        .artists()
        .into_iter()
        .map(|artist| artist.model())
        .collect();
    // Upsert all the artists, returning the artists with their ID's
    debug!("Upserting artists into database");
    let db_artists = db::spotify::upsert_artists(artist_models)
        .await
        .expect("Error upserting artists");
    // Next, convert the recent track albums to their models, using our databases artist IDs and save the albums/album artists
    debug!("Parsing albums from recent tracks");
    let raw_albums_with_artists: Vec<(album::ActiveModel, Vec<artist::Model>)> = recent_tracks
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
                    db_artists
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
    let db_albums_with_artists = db::spotify::upsert_albums_with_artists(raw_albums_with_artists)
        .await
        .expect("Error upserting albums");
    // Lastly, we upsert the tracks, making sure to get the ID's of the tracks, using the album ID's and artist ID's
    debug!("Parsing tracks from recent tracks");
    // Each track should reference an artist and an album, and then use the album to also create an album track
    let raw_tracks_with_albums: Vec<(track::ActiveModel, album::Model)> = recent_tracks
        .into_iter()
        .map(|recent_track| {
            // Get the track active model
            let db_track = recent_track.track.model();
            // Find the album
            let db_album = db_albums_with_artists
                .iter()
                .find(|album| album.title == recent_track.track.album.name)
                .expect("Album not found")
                .to_owned();
            (db_track, db_album)
        })
        .collect();
    // Upsert the tracks with their albums, returning the tracks with their ID's
    let db_tracks_with_albums = db::spotify::upsert_tracks_with_albums(raw_tracks_with_albums)
        .await
        .expect("Error upserting tracks");
    // Return the response
    Ok(Json(CollectionResponse { updated_token }))
}

/// An internal function for collecting recent tracks from Spotify
/// In addition to getting the tracks, this function also handles refreshing the access token if it is invalid
async fn collect_recent_tracks(
    access_token: String,
    refresh_token: Option<String>,
) -> Result<(Vec<RecentTrack>, Option<String>), SpotifyError> {
    // Generate a client for interacting with Spotify
    let client = SpotifyClient::new(access_token).set_refresh_token(refresh_token);
    // Fetch the recent tracks from Spotify
    match client.get_recent_tracks().await {
        Ok(recent_tracks) => Ok((recent_tracks.items.unwrap_or_default(), None)),
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
                    Ok((
                        recent_tracks.items.unwrap_or_default(),
                        Some(new_token.access_token),
                    ))
                }
                _ => Err(spotify_err),
            }
        }
    }
}
