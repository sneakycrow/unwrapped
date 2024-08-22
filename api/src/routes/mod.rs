mod auth;
mod collect;

use crate::assets::Assets;
use axum::{response::Html, routing::get, Router};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
}

pub fn router(state: AppState) -> Router {
    let auth_router = auth::get_auth_router();
    let collect_router = Router::new()
        .route("/", get(index))
        .route("/collect", get(collect::route));
    Router::new()
        .merge(collect_router)
        .with_state(state)
        .merge(auth_router)
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
