use crate::{
    assets::Assets,
    music::spotify::{RecentTrack, SpotifyClient},
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Json, Router,
};

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
}

#[derive(Deserialize, Debug)]
struct Tokens {
    access_token: String,
}

async fn collect(
    State(_state): State<AppState>,
    tokens: Query<Tokens>,
) -> Json<CollectionResponse> {
    let access_token = tokens.access_token.to_owned();
    let response = SpotifyClient::new(access_token)
        .get_recent_tracks()
        .await
        .expect("Failed to collect spotify data");

    // TODO: Save recent tracks to database
    Json(CollectionResponse {
        data: response.items,
    })
}
