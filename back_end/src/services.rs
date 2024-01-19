use axum::{routing::get, Router};

use crate::domain::ApiKey;
use crate::routes::get_calendar;

// fn handle_error() -> (StatusCode, &'static str) {
//     (StatusCode::NOT_FOUND, "Not found!")
// }

// ********
// BACK END
// ********
pub fn backend() -> Router {
    let api_key = ApiKey::new();

    Router::new()
        .route("/api", get(get_calendar))
        .with_state(api_key)
}
