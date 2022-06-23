use core::time::Duration;
use regex::Regex;
use reqwest::{redirect::Policy, Client, ClientBuilder, StatusCode};

pub(crate) mod adfly;
pub(crate) mod adfocus;
pub(crate) mod generic;
pub(crate) mod redirect;
pub(crate) mod refresh;
pub(crate) mod shorturl;
pub(crate) mod surlli;

use futures::future::{ready, TryFutureExt};

use crate::Result;

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

/// Get Page Content if status!=200
pub(crate) async fn from_url_not_200(url: &str, timeout: Option<Duration>) -> Result<String> {
    ready(get_client_builder(timeout).build())
        .and_then(|client| async move {
            client
                .get(url)
                .header(
                    "Accept",
                    "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
                )
                .header("Accept-Language", "en-US,en;q=0.5")
                .header("Cache-Control", "no-cache")
                .send()
                .await
        })
        .err_into()
        .and_then(|response| async move {
            if response.status() == StatusCode::OK {
                Err(crate::error::Error::NoString)
            } else {
                Ok(response.text().await?)
            }
        })
        .await
}

/// get page content irrespective of status code
pub(crate) async fn from_url(url: &str, timeout: Option<Duration>) -> Result<String> {
    ready(get_client_builder(timeout).build())
        .and_then(|client| async move {
            client
                .get(url)
                .header(
                    "Accept",
                    "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
                )
                .header("Accept-Language", "en-US,en;q=0.5")
                .header("Cache-Control", "no-cache")
                .send()
                .await
        })
        .err_into()
        .and_then(|response| async move { Ok(response.text().await?) })
        .await
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
