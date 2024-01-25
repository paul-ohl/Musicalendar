use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Song;

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    items: Vec<CalendarItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CalendarItem {
    date: String,
    song: Song,
}

impl Calendar {
    /// # Errors
    /// Will return an error if the playlist data is not in the correct json format
    pub fn parse(playlist_data: &Value) -> Result<Self> {
        let mut items: Vec<CalendarItem> = Vec::new();
        let Some(songs) = playlist_data["items"].as_array() else {
            anyhow::bail!("Could not retrieve songs")
        };
        for song_infos in songs {
            let date = match song_infos["added_at"].as_str() {
                Some(d) => d.to_owned(),
                None => anyhow::bail!("Could not retrieve song date"),
            };
            let song = Song::parse(&song_infos["track"])?;
            items.push(CalendarItem::new(date, song));
        }
        Ok(Self { items })
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl CalendarItem {
    const fn new(date: String, song: Song) -> Self {
        Self { date, song }
    }
}
