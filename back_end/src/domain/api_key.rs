use std::time::Duration;

use anyhow::{Context, Result};
use serde_json::Value;
use tokio::sync::Mutex;

const SPOTIFY_TOKEN_EXPIRE_TIME: u64 = 3600;
const SPOTIFY_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

#[derive(Debug)]
pub struct ApiKey {
    client: reqwest::Client,
    token: Mutex<SpotifyToken>,
    spotify_identifiers: SpotifyIdentifiers,
}

#[derive(Debug)]
struct SpotifyIdentifiers {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
struct SpotifyToken {
    key: String,
    time_alive: Duration,
}

impl ApiKey {
    #[must_use]
    pub fn new(spotify_client_id: String, spotify_client_secret: String) -> Self {
        let token = SpotifyToken {
            key: String::new(),
            time_alive: Duration::new(0, 0),
        };
        let spotify_identifiers = SpotifyIdentifiers {
            client_id: spotify_client_id,
            client_secret: spotify_client_secret,
        };
        Self {
            token: Mutex::new(token),
            client: reqwest::Client::new(),
            spotify_identifiers,
        }
    }

    #[tracing::instrument]
    pub async fn get_key(&self) -> Result<String> {
        let mut token = self.token.lock().await;
        if token.key.is_empty() || token.time_alive.as_secs() > SPOTIFY_TOKEN_EXPIRE_TIME {
            let request = self
                .client
                .post(SPOTIFY_TOKEN_URL)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(format!(
                    "grant_type=client_credentials&client_id={}&client_secret={}",
                    self.spotify_identifiers.client_id, self.spotify_identifiers.client_secret
                ))
                .send()
                .await;
            if let Ok(response) = request {
                token.key = response
                    .json::<Value>()
                    .await?
                    .get("access_token")
                    .context("Could not get key")?
                    .to_string()
                    .replace('\"', "");
                println!("Generated key: {:?}", token.key);
            } else {
                eprintln!("Could not get key");
                anyhow::bail!("Could not get key");
            }
            token.time_alive = Duration::new(0, 0);
        }
        Ok(token.key.clone())
    }
}
