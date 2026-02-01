// SURL.LI Resolver
use crate::resolvers::{from_url, generic};
use futures::future::{ready, TryFutureExt};
use std::time::Duration;

use crate::{Error, Result};

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs shortened by surl.li.
    //!
    //! This resolver handles surl.li's two-stage redirect process:
    //! 1. First attempts standard HTTP redirect following
    //! 2. If the URL doesn't change, parses HTML for API-based redirect
    //!
    //! # Arguments
    //!
    //! * `url` - The surl.li shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL on success,
    //! or `Err(Error)` if the URL cannot be expanded.
    //!
    //! # Behavior
    //!
    //! - First tries standard HTTP redirect following
    //! - If no expansion occurs, parses HTML for API redirect URL
    //! - Extracts the final URL from api.miniature.io calls
    let expanded_url = generic::unshort(url, timeout).await?;
    Ok(
        if url.ends_with(expanded_url.split("//").last().unwrap_or_default()) {
            match get_from_html(url, timeout).await {
                Ok(u) => u,
                Err(_) => expanded_url,
            }
        } else {
            expanded_url
        },
    )
}

async fn get_from_html(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Extracts the final URL from surl.li's HTML page.
    //!
    //! This function parses the HTML content to find the API call
    //! that contains the actual destination URL.
    //!
    //! # Arguments
    //!
    //! * `url` - The surl.li URL to parse
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the extracted destination URL, or
    //! `Err(Error)` if the URL cannot be extracted.
    //!
    //! # Behavior
    //!
    //! - Fetches the HTML content of the page
    //! - Searches for api.miniature.io URL references
    //! - Extracts the final URL from the API call
    from_url(url, timeout)
        .and_then(|html| {
            ready(
                html.split("api.miniature.io/?url=")
                    .last()
                    .and_then(|r| r.split('"').next())
                    .map(|r| r.to_string())
                    .ok_or(Error::NoString),
            )
        })
        .await
}
