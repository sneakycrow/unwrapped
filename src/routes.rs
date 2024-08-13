use crate::{
    assets::Assets,
    music::spotify::{RecentTrack, SpotifyClient, SpotifyError},
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Json, Router,
};

use tracing::{debug, error};

#[derive(Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/collect", get(collect))
        .with_state(state)
}

async fn index() -> Html<String> {
    let html_string: String = match Assets::get("index.html") {
        Some(file) => {
            let file = file.to_owned();
            String::from_utf8(file.data.into()).unwrap()
        }
        None => "Error".to_string(),
    };

    Html(html_string)
}

#[derive(Serialize)]
pub struct CollectionResponse {
    data: Vec<RecentTrack>,
    updated_token: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Tokens {
    access_token: String,
    refresh_token: Option<String>,
}

/// Collect goes to each of the configured providers, collects the relative data, and saves it to the DB
async fn collect(
    State(_state): State<AppState>,
    tokens: Query<Tokens>,
) -> Result<Json<CollectionResponse>, (StatusCode, String)> {
    // Collect the tokens from the query
    let access_token = tokens.access_token.to_owned();
    let refresh_token = tokens.refresh_token.to_owned();
    // Get the recent tracks from Spotify
    let recent_tracks = collect_recent_tracks(access_token, refresh_token).await;
    match recent_tracks {
        Ok(recent_tracks) => Ok(Json(recent_tracks)),
        Err(spotify_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error collecting spotify data: {:?}", spotify_err),
        )),
    }
}

/// An internal function for collecting recent tracks from Spotify
/// In addition to getting the tracks, this function also handles refreshing the access token if it is invalid
async fn collect_recent_tracks(
    access_token: String,
    refresh_token: Option<String>,
) -> Result<CollectionResponse, SpotifyError> {
    // Generate a client for interacting with Spotify
    let client = SpotifyClient::new(access_token).set_refresh_token(refresh_token);
    // Fetch the recent tracks from Spotify
    match client.get_recent_tracks().await {
        Ok(recent_tracks) => {
            debug!("Successfully collected spotify data");
            Ok(CollectionResponse {
                data: recent_tracks.items.unwrap_or_default(),
                updated_token: None,
            })
        }
        Err(spotify_err) => {
            match spotify_err.status {
                // If the error is an invalid token error, try to get a new access token
                401 => {
                    debug!("Invalid Token error, Attempting to get a new access token");
                    // First get the new access token
                    let new_token = client.refresh_access_token().await.map_err(|err| {
                        error!(
                            "Error trying to recover from failed access token: {:?}",
                            err
                        );
                        (
                            StatusCode::UNPROCESSABLE_ENTITY,
                            "Could not refresh access token".to_string(),
                        )
                    })?;
                    // Update the client with the new access token and try to get the recent tracks again
                    let recent_tracks = client
                        .set_access_token(new_token.access_token.clone())
                        .get_recent_tracks()
                        .await
                        .map_err(|err| {
                            error!("Error collecting spotify data after recovery: {:?}", err);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Error collecting spotify data".to_string(),
                            )
                        })?;
                    // Return the recent tracks and the new access token
                    Ok(CollectionResponse {
                        data: recent_tracks,
                        updated_token: Some(new_token.access_token),
                    })
                }
                // Any other errors aren't specifically handled, so we log the error and return a 500
                _ => {
                    error!("Error collecting spotify data: {:?}", spotify_err);
                    Err(spotify_err)
                }
            }
        }
    }
}
