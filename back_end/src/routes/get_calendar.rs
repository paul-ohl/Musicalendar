use axum::extract::{Query, State};
use serde::Deserialize;
use serde_json::{json, Value};
// use serde_json::{json, Value};

use crate::domain::{ApiKey, PlaylistId};

#[derive(Debug, Deserialize)]
pub struct Params {
    playlist_link: String,
}

pub async fn get_calendar(State(api_key): State<ApiKey>, Query(params): Query<Params>) -> String {
    if let Ok(id) = PlaylistId::parse(&params.playlist_link) {
        spotify_get_from_link(api_key, id).await.to_string()
    } else {
        String::from("Could not find playlist from this id")
    }
}

async fn spotify_get_from_link(api_key: ApiKey, playlist_id: PlaylistId) -> Value {
    let spotify_token = api_key.get_key().await;
    let client = reqwest::Client::new();

    let request = client
        .get(format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            playlist_id.as_ref()
        ))
        .header("Authorization", format!("Bearer {spotify_token}"))
        .send()
        .await;
    match request {
        Ok(response) => response.json::<serde_json::Value>().await.unwrap(),
        Err(e) => json!({"result": "failed", "reason": e.to_string()}),
    }
}
