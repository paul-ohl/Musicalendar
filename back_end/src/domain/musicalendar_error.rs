use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct MusicalendarError(anyhow::Error);

impl From<anyhow::Error> for MusicalendarError {
    fn from(e: anyhow::Error) -> Self {
        Self(e)
    }
}

impl IntoResponse for MusicalendarError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
