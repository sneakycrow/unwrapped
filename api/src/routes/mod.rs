mod auth;
mod save_recent_tracks;

use crate::assets::Assets;
use axum::{response::Html, routing::get, Router};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/save-recent-tracks", get(save_recent_tracks::route))
        .route("/login", get(auth::login))
        .route("/auth/spotify", get(auth::spotify_auth))
        .route("/auth/spotify/callback", get(auth::spotify_auth_callback))
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
