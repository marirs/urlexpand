use core::time::Duration;
use regex::Regex;
use reqwest::{redirect::Policy, Client, ClientBuilder, StatusCode};

pub(crate) mod adfly;
pub(crate) mod adfocus;
pub(crate) mod generic;
pub(crate) mod linkedin;
pub(crate) mod redirect;
pub(crate) mod refresh;
pub(crate) mod shorturl;
pub(crate) mod surlli;
pub(crate) mod urlshortdev;

use futures::future::{ready, TryFutureExt};

use crate::Result;

static  UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0";

/// Get the reqwest ClientBuilder with common configuration
pub(crate) fn get_client_builder(timeout: Option<Duration>) -> ClientBuilder {
    //! Creates a configured reqwest ClientBuilder for HTTP requests.
    //!
    //! This function sets up common HTTP client configuration used across
    //! all resolvers, including user agent, SSL certificate handling,
    //! and optional timeout settings.
    //!
    //! # Arguments
    //!
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns a configured `ClientBuilder` ready for use.
    //!
    //! # Configuration
    //!
    //! - Sets a realistic browser User-Agent string
    //! - Accepts invalid SSL certificates (for some shorteners)
    //! - Applies timeout if provided
    match timeout {
        Some(x) => Client::builder().timeout(x),
        None => Client::builder(),
    }
    .user_agent(UA)
    .danger_accept_invalid_certs(true)
}

/// Reqwest Custom Redirect Policy
pub(crate) fn custom_redirect_policy() -> Policy {
    //! Creates a custom redirect policy for safe URL expansion.
    //!
    //! This policy prevents potentially malicious cross-domain redirects
    //! while allowing legitimate same-domain redirect chains.
    //!
    //! # Returns
    //!
    //! Returns a `Policy` that stops redirects when the domain changes.
    //!
    //! # Behavior
    //!
    //! - Allows redirects within the same domain
    //! - Stops redirects when the domain changes from the original
    //! - Prevents potential redirect loops or malicious chains
    Policy::custom(|attempt| {
        let n_attempt = attempt.previous().len();
        if attempt.previous()[0].host() != attempt.previous()[n_attempt - 1].host() {
            attempt.stop()
        } else {
            attempt.follow()
        }
    })
}

/// Get Page Content if status != 200
pub(crate) async fn from_url_not_200(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Fetches HTML content from URLs that typically return non-200 status codes.
    //!
    //! Many URL shorteners return non-200 status codes (like 302, 403, etc.)
    //! before showing their redirect pages. This function specifically handles
    //! those cases by returning the content when the status is not 200 OK.
    //!
    //! # Arguments
    //!
    //! * `url` - The URL to fetch content from
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the HTML content if status != 200,
    //! or `Err(Error::NoString)` if status is 200 or if the request fails.
    //!
    //! # Behavior
    //!
    //! - Sets appropriate headers for HTML content
    //! - Returns content only when status code is not 200
    //! - Returns error for 200 status (expected for these shorteners)
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

/// Get page content irrespective of status code
pub(crate) async fn from_url(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Fetches HTML content from URLs regardless of HTTP status code.
    //!
    //! This function is used when we need to examine the actual HTML content
    //! of a page, regardless of whether the request was successful or not.
    //! It's commonly used for parsing redirect pages or error pages.
    //!
    //! # Arguments
    //!
    //! * `url` - The URL to fetch content from
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the HTML content of the page,
    //! or `Err(Error)` if the request fails.
    //!
    //! # Behavior
    //!
    //! - Sets appropriate headers for HTML content
    //! - Returns content for any status code
    //! - Used for parsing pages that may contain redirect information
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
    //! Extracts the first capture group from text using a regex pattern.
    //!
    //! This helper function is used throughout resolvers to extract URLs
    //! or other information from HTML content using regular expressions.
    //!
    //! # Arguments
    //!
    //! * `txt` - The text to search within
    //! * `p` - The regex pattern to use for matching
    //!
    //! # Returns
    //!
    //! Returns `Some(String)` with the first capture group if a match is found,
    //! or `None` if no match is found or the regex is invalid.
    //!
    //! # Behavior
    //!
    //! - Compiles the regex pattern
    //! - Returns the first capture group (skipping the full match)
    //! - Handles regex compilation errors gracefully
    Regex::new(p)
        .ok()
        .and_then(|pattern| {
            pattern
                .captures(txt)
                .and_then(|c| c.iter().skip(1).flatten().next())
        })
        .map(|x| x.as_str().into())
}
