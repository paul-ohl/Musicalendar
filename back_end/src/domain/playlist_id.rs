use anyhow::{bail, Result};
use regex::Regex;

#[derive(Debug)]
pub struct PlaylistId(String);

impl PlaylistId {
    /// # Errors
    /// Will return an error if the link does not contain a valid playlist id
    ///
    /// # Panics
    /// Will panic if the regex initialisation fails, which will not happen.
    pub fn parse(link: &str) -> Result<Self, anyhow::Error> {
        let link_regex = Regex::new(r"([\w\d]+$)").expect("There is an error in the Regex");
        let Some(capture) = link_regex.captures(remove_after_char(link, '?')) else {
            bail!("Could not find id in link: {}", link);
        };
        Ok(Self(capture[1].to_owned()))
    }
}

impl AsRef<str> for PlaylistId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn remove_after_char(s: &str, c: char) -> &str {
    s.split(c).next().map_or(s, |s| s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_links() {
        let valid_link = "2zgtmblCe6e3eUSoBSSAJW";
        let link = PlaylistId::parse("https://open.spotify.com/playlist/2zgtmblCe6e3eUSoBSSAJW")
            .expect("cannot fail.");
        assert_eq!(valid_link, link.as_ref());
        let link = PlaylistId::parse(
            "https://open.spotify.com/playlist/2zgtmblCe6e3eUSoBSSAJW?si=0edd878c7a984a77",
        )
        .expect("cannot fail.");
        assert_eq!(valid_link, link.as_ref());
        let link = PlaylistId::parse("http://open.spotify.com/playlist/2zgtmblCe6e3eUSoBSSAJW")
            .expect("cannot fail.");
        assert_eq!(valid_link, link.as_ref());
        let link =
            PlaylistId::parse("spotify:playlist:2zgtmblCe6e3eUSoBSSAJW").expect("cannot fail.");
        assert_eq!(valid_link, link.as_ref());
        let link =
            PlaylistId::parse("https://spoti.fi/2zgtmblCe6e3eUSoBSSAJW").expect("cannot fail.");
        assert_eq!(valid_link, link.as_ref());
        let link = PlaylistId::parse("2zgtmblCe6e3eUSoBSSAJW").expect("cannot fail.");
        assert_eq!(valid_link, link.as_ref());
    }

    #[test]
    fn invalid_links() {
        let invalid_link = PlaylistId::parse("https://open.spotify.com/playlist/");
        assert!(invalid_link.is_err());
    }
}
