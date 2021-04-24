use core::time::Duration;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};

pub(crate) mod adfly;
pub(crate) mod adfocus;
pub(crate) mod generic;
pub(crate) mod meta_refresh;
pub(crate) mod nowlinks;
pub(crate) mod rlu;
pub(crate) mod shorturl;
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

/// Reqwest Custom Redirect Policy
pub(crate) fn custom_redirect_policy() -> Policy {
    Policy::custom(|attempt| {
        let n_attempt = attempt.previous().len();
        if attempt.previous()[0].host() != attempt.previous()[n_attempt - 1].host() {
            attempt.stop()
        } else {
            attempt.follow()
        }
    })
}

/// Get Page Content if status==200
pub(crate) fn from_url(url: &str, timeout: Option<Duration>) -> Option<String> {
    let client = match get_client_builder(timeout).build() {
        Ok(c) => c,
        Err(_) => return None,
    };

    let response = match client
        .get(url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Cache-Control", "no-cache")
        .send()
    {
        Ok(r) => r,
        Err(_) => return None
    };

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
