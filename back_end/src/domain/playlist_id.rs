use anyhow::{bail, Result};
use regex::Regex;

pub struct PlaylistId(String);

impl PlaylistId {
    pub fn parse(link: &str) -> Result<Self> {
        let link_regex = Regex::new(r"([\w\d]+$)").unwrap();
        let Some(capture) = link_regex.captures(link) else {
            bail!("Could not find id in link: {}", link);
        };
        Ok(PlaylistId(capture[1].to_owned()))
    }
}

impl AsRef<str> for PlaylistId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_links() {
        let valid_link = "2zgtmblCe6e3eUSoBSSAJW";
        let link =
            PlaylistId::parse("https://open.spotify.com/playlist/2zgtmblCe6e3eUSoBSSAJW").unwrap();
        assert_eq!(valid_link, link.as_ref());
        let link =
            PlaylistId::parse("http://open.spotify.com/playlist/2zgtmblCe6e3eUSoBSSAJW").unwrap();
        assert_eq!(valid_link, link.as_ref());
        let link = PlaylistId::parse("spotify:playlist:2zgtmblCe6e3eUSoBSSAJW").unwrap();
        assert_eq!(valid_link, link.as_ref());
        let link = PlaylistId::parse("https://spoti.fi/2zgtmblCe6e3eUSoBSSAJW").unwrap();
        assert_eq!(valid_link, link.as_ref());
        let link = PlaylistId::parse("2zgtmblCe6e3eUSoBSSAJW").unwrap();
        assert_eq!(valid_link, link.as_ref());
    }

    #[test]
    fn invalid_links() {
        let invalid_link = PlaylistId::parse("https://open.spotify.com/playlist/");
        assert!(invalid_link.is_err());
    }
}
