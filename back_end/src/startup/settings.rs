use config::Config as ConfigBuilder;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
    pub host: String,
    pub port: u16,
}

/// # Panics
/// Panics if the config could not be built or deserialized, meaning the config file is likely missing or malformed.
#[must_use]
pub fn build() -> Settings {
    ConfigBuilder::builder()
        // Add in `./settings.toml`
        .add_source(config::File::with_name("settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .expect("Could not build config.")
        .try_deserialize()
        .expect("Could not deserialize config.")
}
