use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use lib::{
    db::user::{create_user_with_account, CreateUserOptions},
    music::spotify::client::SpotifyClient,
};
use serde::Deserialize;
use tracing::error;

use super::AppState;

/// Redirects to the Spotify login page using the appropriate scopes
pub async fn spotify_auth() -> impl IntoResponse {
    let auth_url = SpotifyClient::get_authorize_url();
    Redirect::to(&auth_url.to_string())
}

/// Query parameters from the Spotify callback
#[derive(Deserialize, Debug)]
pub struct SpotifyCallbackQuery {
    code: String,
}

/// Callback from Spotify after the user has logged in
/// This functions upserts the user into the database
pub async fn spotify_auth_callback(
    query: Query<SpotifyCallbackQuery>,
    state: State<AppState>,
) -> impl IntoResponse {
    // Using the code from the query, request an access token from Spotify
    // If successful, upsert the user into the database
    let mut spotify_client = SpotifyClient::new();
    spotify_client
        .get_tokens(query.code.to_owned())
        .await
        .map_err(|e| {
            error!("Failed to get Spotify tokens: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get Spotify tokens".to_string(),
            )
        })
        .unwrap();
    // After parsing the account tokens, upsert the user into the database
    let _user = create_user_with_account(
        &state.connection,
        CreateUserOptions {
            email: "some_email".to_string(),
            name: "some_name".to_string(),
            provider_id: "some_id".to_string(),
            access_token: spotify_client.access_token.expect("No access token"),
            refresh_token: spotify_client.refresh_token.expect("No refresh token"),
            provider: "spotify".to_string(),
        },
    )
    .await
    .map_err(|e| {
        error!("Failed to create user with account: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create user with account".to_string(),
        )
    })
    .unwrap();

    "Some Success".to_string()
}

pub async fn login() -> impl IntoResponse {
    "Login"
}
