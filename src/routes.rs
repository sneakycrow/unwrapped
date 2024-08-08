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

pub fn router(pool: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/collect", get(collect))
        .with_state(pool)
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
    State(_pool): State<DatabaseConnection>,
    tokens: Query<Tokens>,
) -> Json<CollectionResponse> {
    let access_token = tokens.access_token.to_owned();
    let tracks = SpotifyClient::new(access_token)
        .get_recent_tracks()
        .await
        .expect("Failed to fetch recent tracks");

    Json(CollectionResponse { data: tracks.items })
}
