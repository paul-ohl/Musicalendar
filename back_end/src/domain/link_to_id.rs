use anyhow::{bail, Result};
use regex::Regex;

pub fn convert(link: &str) -> Result<String> {
    let link_regex = Regex::new(r"([\w\d]+$)").unwrap();
    let Some(capture) = link_regex.captures(link) else {
        bail!("Could not find id in link: {}", link);
    };
    Ok(capture[1].to_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_links() {
        assert_eq!(
            String::from("2zgtmblCe6e3eUSoBSSAJW"),
            convert("https://open.spotify.com/playlist/2zgtmblCe6e3eUSoBSSAJW").unwrap()
        );
        assert_eq!(
            String::from("2zgtmblCe6e3eUSoBSSAJW"),
            convert("http://open.spotify.com/playlist/2zgtmblCe6e3eUSoBSSAJW").unwrap()
        );
        assert_eq!(
            String::from("2zgtmblCe6e3eUSoBSSAJW"),
            convert("spotify:playlist:2zgtmblCe6e3eUSoBSSAJW").unwrap()
        );
        assert_eq!(
            String::from("2zgtmblCe6e3eUSoBSSAJW"),
            convert("https://spoti.fi/2zgtmblCe6e3eUSoBSSAJW").unwrap()
        );
        assert_eq!(
            String::from("2zgtmblCe6e3eUSoBSSAJW"),
            convert("2zgtmblCe6e3eUSoBSSAJW").unwrap()
        );
    }

    #[test]
    fn invalid_links() {
        assert!(convert("https://open.spotify.com/playlist/").is_err());
    }
}
