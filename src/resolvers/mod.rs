use core::time::Duration;
use regex::Regex;
use reqwest::blocking::Client;

pub(crate) mod generic;
pub(crate) mod twitter;
pub(crate) mod tinyurl;

/// user agent string
static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.72 Safari/537.36";

/// Build the reqwest client
pub(crate) fn build_client(timeout: Option<Duration>) -> Option<Client>{
    let client = if timeout == None {
        match Client::builder()
            .danger_accept_invalid_certs(true)
            .user_agent(USER_AGENT)
            .build() {
            Ok(x) => x,
            Err(_) => return None
        }
    } else {
        let timeout_duration = match timeout {
            Some(x) => x,
            None => {
                return None;
            }
        };
        match Client::builder()
            .timeout(timeout_duration)
            .danger_accept_invalid_certs(true)
            .user_agent(USER_AGENT)
            .build() {
            Ok(x) => x,
            Err(_) => return None
        }
    };

    Some(client)
}

/// Extract text from regex pattern
pub(crate) fn from_re_pattern(txt: &str, p: &str) -> Option<String> {
    let pattern = Regex::new(p).unwrap();
    match pattern.captures(txt) {
        Some(c) => {
            if c.len() > 0 {
                Some(c[1].to_string())
            } else {
                Some(c[0].to_string())
            }
        },
        None => return None
    }
}