// LinkedIn (lnkd.in) Resolver
// LinkedIn is aggressive with anti-scraping and has two behaviors:
// 1. Direct HTTP redirect (most common)
// 2. Interstitial warning page with URL in HTML (when flagged/rate-limited)
// We try both approaches for robustness

use crate::resolvers::{from_url, generic};
use futures::future::{ready, TryFutureExt};
use std::time::Duration;

use crate::{Error, Result};

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands LinkedIn shortened URLs (lnkd.in).
    //!
    //! LinkedIn uses two different redirect mechanisms:
    //! 1. Standard HTTP redirects (most common)
    //! 2. Interstitial warning pages with JavaScript redirects (when flagged/rate-limited)
    //!
    //! This resolver tries both approaches for robustness.
    //!
    //! # Arguments
    //!
    //! * `url` - The LinkedIn shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL on success,
    //! or `Err(Error)` if the URL cannot be expanded.
    //!
    //! # Behavior
    //!
    //! - First attempts standard HTTP redirect following
    //! - If still on LinkedIn domain, parses the interstitial page
    //! - Extracts the final URL from the warning page HTML
    //! - Returns the expanded URL or falls back to the redirect result
    // First try standard HTTP redirect (most common LinkedIn behavior)
    let expanded_url = generic::unshort(url, timeout).await?;

    // If we're still on LinkedIn domain, try parsing the interstitial page
    Ok(
        if expanded_url.contains("linkedin.com") || expanded_url.contains("lnkd.in") {
            get_from_html(url, timeout).await.unwrap_or_else(|_| expanded_url)
        } else {
            expanded_url
        },
    )
}

async fn get_from_html(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Extracts the final URL from LinkedIn's interstitial warning page.
    //!
    //! This function parses the HTML content of LinkedIn's warning page
    //! to find the external link that the user intended to visit.
    //!
    //! # Arguments
    //!
    //! * `url` - The LinkedIn URL that may show an interstitial page
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the extracted destination URL, or
    //! `Err(Error)` if the URL cannot be extracted from the page.
    //!
    //! # Behavior
    //!
    //! - Fetches the HTML content of the URL
    //! - Searches for the external URL link in the warning page
    //! - Extracts the href attribute from the tracking link
    from_url(url, timeout)
        .and_then(|html| {
            ready(
                // Parse the interstitial warning page
                html.split("data-tracking-control-name=\"external_url_click\"")
                    .nth(1)
                    .and_then(|r| r.split("href=\"").nth(1))
                    .and_then(|r| r.split("\">").next())
                    .map(|r| r.to_string())
                    .ok_or(Error::NoString),
            )
        })
        .await
}
