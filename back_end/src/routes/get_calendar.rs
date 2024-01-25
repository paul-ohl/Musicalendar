use anyhow::Result;
use axum::{extract::State, Form};
use serde::Deserialize;
use serde_json::Value;
use std::{convert::Into, sync::Arc};
use tracing::instrument;

use crate::domain::{ApiKey, Calendar, MusicalendarError, PlaylistId};

#[derive(Debug, Deserialize)]
pub struct Params {
    playlist_link: String,
}

/// # Errors
/// Will return an error if any of the subfunctions fail, meaning:
/// - The playlist link is invalid
/// - The playlist data is not in the correct json format
pub async fn get_calendar(
    State(api_key): State<Arc<ApiKey>>,
    Form(params): Form<Params>,
) -> Result<String, MusicalendarError> {
    let id = PlaylistId::parse(&params.playlist_link)?;
    calendar_from_id(api_key, id).await.map_err(Into::into)
}

#[instrument]
async fn calendar_from_id(api_key: Arc<ApiKey>, playlist_id: PlaylistId) -> Result<String> {
    let spotify_token = api_key.get_key().await?;
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            playlist_id.as_ref()
        ))
        .header("Authorization", format!("Bearer {spotify_token}"))
        .send()
        .await?;
    let playlist = response.json::<Value>().await?;

    let calendar = Calendar::parse(&playlist)?;

    serde_json::to_string(&calendar).map_err(Into::into)
}
