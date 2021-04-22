use core::time::Duration;
use regex::Regex;
use reqwest::blocking::{Client, ClientBuilder};

pub(crate) mod generic;
pub(crate) mod tinyurl;

/// get the reqwest ClientBuilder
pub(crate) fn get_client_builder(timeout: Option<Duration>) -> ClientBuilder {
    match timeout {
        Some(x) => {
            Client::builder()
                .timeout(x)
                .danger_accept_invalid_certs(true)
        }
        None => {
            Client::builder()
                .danger_accept_invalid_certs(true)
        }
    }
}

/// Extract text from regex pattern
pub(crate) fn from_re_pattern(txt: &str, p: &str) -> Option<String> {
    let pattern = match Regex::new(p) {
        Ok(p) => p,
        Err(_) => return None
    };
    pattern.captures(txt).map(|c| {
        if c.len() > 0 {
            c[1].to_string()
        } else {
            c[0].to_string()
        }
    })
}
