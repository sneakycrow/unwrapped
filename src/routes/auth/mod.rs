use axum::{response::IntoResponse, routing::get, Router};

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

async fn spotify_auth() -> impl IntoResponse {
    "Spotify Auth"
}

async fn spotify_auth_callback() -> impl IntoResponse {
    "Spotify Auth Callback"
}

async fn login() -> impl IntoResponse {
    "Login"
}
