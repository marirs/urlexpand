use core::time::Duration;
use regex::Regex;
use reqwest::blocking::{Client, ClientBuilder};

pub(crate) mod generic;
pub(crate) mod google;
pub(crate) mod tinyurl;
pub(crate) mod twitter;

/// user agent string
static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.72 Safari/537.36";

/// Build the reqwest client
pub(crate) fn build_client_builder(timeout: Option<Duration>) -> ClientBuilder {
    match timeout {
        Some(x) => {
            Client::builder()
                .timeout(x)
                .danger_accept_invalid_certs(true)
                .user_agent(USER_AGENT)
        }
        None => {
            Client::builder().danger_accept_invalid_certs(true)
                .user_agent(USER_AGENT)
        }
    }
}

/// Extract text from regex pattern
pub(crate) fn from_re_pattern(txt: &str, p: &str) -> Option<String> {
    let pattern = Regex::new(p).unwrap();
    pattern.captures(txt).map(|c| {
        if c.len() > 0 {
            c[1].to_string()
        } else {
            c[0].to_string()
        }
    })
}
