mod assets;
mod routes;

use lib::db;
use migration::{Migrator, MigratorTrait};
use tokio::net::TcpListener;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize trace subscriber
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "unwrapped=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Initialize database connection and migrate
    let connection = db::get_connection()
        .await
        .expect("Failed to connect to database");
    Migrator::up(&connection, None)
        .await
        .expect("Failed to migrate database");
    // Construct shared app state
    let state = routes::AppState { connection };
    // Initialize the API
    let app = routes::router(state).layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new())
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Micros),
            ),
    );
    // Start the API
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
