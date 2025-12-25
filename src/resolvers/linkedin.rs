// LinkedIn (lnkd.in) Resolver
// LinkedIn is aggressive with anti-scraping and has two behaviors:
// 1. Direct HTTP redirect (most common)
// 2. Interstitial warning page with URL in HTML (when flagged/rate-limited)
// We try both approaches for robustness

use crate::resolvers::{from_url, generic};
use futures::future::{ready, TryFutureExt};
use std::time::Duration;

use crate::{Error, Result};

/// LinkedIn URL Expander
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    // First try standard HTTP redirect (most common LinkedIn behavior)
    let expanded_url = generic::unshort(url, timeout).await?;

    // If we're still on LinkedIn domain, try parsing the interstitial page
    Ok(
        if expanded_url.contains("linkedin.com") || expanded_url.contains("lnkd.in") {
            match get_from_html(url, timeout).await {
                Ok(u) => u,
                Err(_) => expanded_url, // Fallback to whatever generic gave us
            }
        } else {
            expanded_url
        },
    )
}

async fn get_from_html(url: &str, timeout: Option<Duration>) -> Result<String> {
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
