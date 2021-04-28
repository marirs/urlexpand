use core::time::Duration;
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
    StatusCode,
};

pub(crate) mod adfly;
pub(crate) mod adfocus;
pub(crate) mod generic;
pub(crate) mod redirect;
pub(crate) mod refresh;
pub(crate) mod shorturl;

static UA: &str = "curl/7.72.0";

/// get the reqwest ClientBuilder
pub(crate) fn get_client_builder(timeout: Option<Duration>) -> ClientBuilder {
    match timeout {
        Some(x) => Client::builder().timeout(x),
        None => Client::builder(),
    }
    .user_agent(UA)
    .danger_accept_invalid_certs(true)
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
    get_client_builder(timeout)
        .build()
        .and_then(|client| {
            client
                .get(url)
                .header(
                    "Accept",
                    "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
                )
                .header("Accept-Language", "en-US,en;q=0.5")
                .header("Cache-Control", "no-cache")
                .send()
        })
        .ok()
        .filter(|response| response.status() == StatusCode::OK)
        .and_then(|response| response.text().ok())
}

/// Extract text from regex pattern
fn from_re(txt: &str, p: &str) -> Option<String> {
    Regex::new(p)
        .ok()
        .and_then(|pattern| {
            pattern
                .captures(txt)
                .and_then(|c| c.iter().skip(1).flatten().next())
        })
        .map(|x| x.as_str().into())
}
