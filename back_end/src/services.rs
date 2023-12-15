use axum::{handler::HandlerWithoutStateExt, http::StatusCode, routing::get, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::api_key::ApiKey;
use crate::routes::playlist_link::generate;
use crate::FRONT_PUBLIC;

// *********
// FRONT END
// *********
pub fn frontend() -> Router {
    Router::new()
        .fallback_service(
            ServeDir::new(FRONT_PUBLIC).not_found_service(handle_error().into_service()),
        )
        .layer(TraceLayer::new_for_http())
}

fn handle_error() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found!")
}

// ********
// BACK END
// ********
pub fn backend() -> Router {
    let api_key = ApiKey::new();

    Router::new()
        .route("/api", get(generate))
        .with_state(api_key)
}
