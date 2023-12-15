use axum::extract::{Query, State};
use serde::Deserialize;
use serde_json::{json, Value};
// use serde_json::{json, Value};

use crate::{api_key::ApiKey, domain::link_to_id};

#[derive(Debug, Deserialize)]
pub struct Params {
    playlist_link: String,
}

pub async fn generate(State(api_key): State<ApiKey>, Query(params): Query<Params>) -> String {
    if let Ok(id) = link_to_id::convert(&params.playlist_link) {
        spotify_get_from_link(api_key, id).await.to_string()
    } else {
        String::from("Could not find playlist from this id")
    }
}

async fn spotify_get_from_link(api_key: ApiKey, id: String) -> Value {
    let spotify_token = api_key.get_key().await;
    json!("Not implemented yet!")
}
