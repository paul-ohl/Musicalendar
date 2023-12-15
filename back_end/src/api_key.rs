use std::time::Duration;

use axum::Json;

const SPOTIFY_TOKEN_EXPIRE_TIME: u64 = 3600;

#[derive(Clone)]
pub struct ApiKey {
    key: String,
    time_alive: Duration,
}

impl ApiKey {
    pub fn new() -> Self {
        ApiKey {
            key: "Hello, World!".to_string(),
            time_alive: Duration::new(0, 0),
        }
    }

    pub async fn get_key(&self) -> String {
        if self.key.is_empty() || self.time_alive.as_secs() > SPOTIFY_TOKEN_EXPIRE_TIME {
            let client = reqwest::Client::new();
            let request = client.post("https://accounts.spotify.com/api/token")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body("grant_type=client_credentials&client_id=4779c329d87d40e2b958d51d413e19c6&client_secret=f2e2fd71ca374704a4b1586099171b5f")
                .send()
                .await;
            if let Ok(response) = request {
                return response.text().await.unwrap();
            }
        }
        String::new()
    }
}
