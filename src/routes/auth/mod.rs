use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use surf::{http::mime, Body, Url};
use tracing::error;

pub fn get_auth_router() -> Router {
    let spotify_routes = get_spotify_auth_router();
    Router::new()
        .route("/login", get(login))
        .merge(spotify_routes)
}

pub fn get_spotify_auth_router() -> Router {
    Router::new()
        .route("/auth/spotify", get(spotify_auth))
        .route("/auth/spotify/callback", get(spotify_auth_callback))
}

/// Redirects to the Spotify login page using the appropriate scopes
async fn spotify_auth() -> impl IntoResponse {
    const BASE_URL: &'static str = "https://accounts.spotify.com/authorize?";
    let creds = SpotifyOAuthSettings::from_env();
    let redirect_url = Url::parse_with_params(
        BASE_URL,
        &[
            ("response_type", "code"),
            ("client_id", &creds.client_id),
            ("scope", &creds.scopes),
            ("redirect_uri", &creds.redirect_uri),
        ],
    )
    .expect("Failed to construct Spotify OAuth URL");
    Redirect::to(&redirect_url.to_string())
}

/// Query parameters from the Spotify callback
#[derive(Deserialize, Debug)]
struct SpotifyCallbackQuery {
    code: String,
    state: Option<String>,
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

/// Callback from Spotify after the user has logged in
/// This functions upserts the user into the database
async fn spotify_auth_callback(query: Query<SpotifyCallbackQuery>) -> impl IntoResponse {
    // Using the code from the query, request an access token from Spotify
    // If successful, upsert the user into the database
    const BASE_URL: &'static str = "https://accounts.spotify.com/api/token";
    let creds = SpotifyOAuthSettings::from_env();
    let auth_header =
        BASE64_STANDARD.encode(format!("{}:{}", creds.client_id, creds.client_secret));
    let res: SpotifyTokenResponse = surf::post(BASE_URL)
        .header("Authorization", format!("Basic {}", auth_header))
        .content_type(mime::FORM)
        .body(SpotifyTokenRequest {
            code: query.code.clone(),
            redirect_uri: creds.redirect_uri.clone(),
            grant_type: "authorization_code".to_string(),
        })
        .recv_json()
        .await
        .map_err(|e| error!("Failed to request access token from Spotify: {}", e))
        .unwrap();
    Json(res)
}

async fn login() -> impl IntoResponse {
    "Login"
}

struct SpotifyOAuthSettings {
    client_id: String,
    client_secret: String,
    scopes: String,
    redirect_uri: String,
}

impl SpotifyOAuthSettings {
    fn from_env() -> Self {
        // Get the values from environment variables
        let client_id = std::env::var("SPOTIFY_ID").expect("SPOTIFY_CLIENT_ID not set");
        let client_secret = std::env::var("SPOTIFY_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");
        let scopes = std::env::var("SPOTIFY_SCOPES").expect("SPOTIFY_SCOPES not set");
        let redirect_uri =
            std::env::var("SPOTIFY_REDIRECT_URI").expect("SPOTIFY_REDIRECT_URI not set");
        Self {
            client_id,
            client_secret,
            scopes,
            redirect_uri,
        }
    }
}
