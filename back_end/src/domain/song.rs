use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    name: String,
    album: String,
    artist: String,
    link: String,
    cover_image: String,
}

impl Song {
    #[allow(clippy::unnecessary_wraps)]
    pub fn parse(song_data: &Value) -> Result<Self> {
        let name = match song_data["name"].as_str() {
            Some(n) => n.to_owned(),
            None => anyhow::bail!("Could not retrieve song name"),
        };
        let album = match song_data["album"]["name"].as_str() {
            Some(a) => a.to_owned(),
            None => anyhow::bail!("Could not retrieve song artist"),
        };
        let artist = match song_data["artists"][0]["name"].as_str() {
            Some(a) => a.to_owned(),
            None => anyhow::bail!("Could not retrieve song artist"),
        };
        let link = match song_data["href"].as_str() {
            Some(l) => l.to_owned(),
            None => anyhow::bail!("Could not retrieve song link"),
        };
        let cover_image = song_data["album"]["images"][0]["url"]
            .as_str()
            .unwrap_or("")
            .to_owned();
        Ok(Self {
            name,
            album,
            artist,
            link,
            cover_image,
        })
    }
}

impl Default for Song {
    fn default() -> Self {
        Self {
            name: "Flying Whales".to_string(),
            album: "From Mars to Sirius".to_string(),
            artist: "Gojira".to_string(),
            link: "no_link".to_string(),
            cover_image: "no either".to_string(),
        }
    }
}
