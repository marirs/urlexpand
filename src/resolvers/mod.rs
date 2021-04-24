use core::time::Duration;
use regex::Regex;
use reqwest::blocking::{Client, ClientBuilder};

pub(crate) mod adfly;
pub(crate) mod generic;
pub(crate) mod nowlinks;
pub(crate) mod rlu;
pub(crate) mod tinyurl;

static UA: &str = "curl/7.72.0";

/// get the reqwest ClientBuilder
pub(crate) fn get_client_builder(timeout: Option<Duration>) -> ClientBuilder {
    match timeout {
        Some(x) => {
            Client::builder()
                .timeout(x)
                .user_agent(UA)
                .danger_accept_invalid_certs(true)
        }
        None => {
            Client::builder()
                .user_agent(UA)
                .danger_accept_invalid_certs(true)
        }
    }
}

/// Get Page Content if status==200
pub(crate) fn from_url(url: &str, timeout: Option<Duration>) -> Option<String> {
    let client = match get_client_builder(timeout).build() {
        Ok(c) => c,
        Err(_) => return None,
    };
    let response = client.get(url).send().ok().unwrap();
    let status = response.status();
    if status == 200 {
        let text = match response.text() {
            Ok(t) => t,
            _ => return None,
        };

        return Some(text)
    }

    None
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
