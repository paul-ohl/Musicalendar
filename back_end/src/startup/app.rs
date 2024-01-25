use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::domain::ApiKey;
use crate::routes::get_calendar;

use super::Settings;

pub fn app(config: Settings) -> Router {
    Router::new().nest("/api", backend(config))
}

// fn handle_error() -> (StatusCode, &'static str) {
//     (StatusCode::NOT_FOUND, "Not found!")
// }

// ********
// BACK END
// ********
fn backend(config: Settings) -> Router {
    let api_key = Arc::new(ApiKey::new(
        config.spotify_client_id.clone(),
        config.spotify_client_secret.clone(),
    ));

    Router::new()
        .route("/calendar", post(get_calendar))
        .route("/health_check", get(|| async { "OK" }))
        .with_state(api_key)
        .with_state(config)
}
