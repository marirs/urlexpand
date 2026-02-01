// Generic Resolver
use std::time::Duration;

use super::{custom_redirect_policy, get_client_builder};

use futures::future::{ready, TryFutureExt};

use crate::Result;

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs using standard HTTP redirect following.
    //!
    //! This resolver handles the majority of URL shorteners that rely on
    //! standard HTTP 3xx redirects. It follows redirect chains while applying
    //! a custom redirect policy to prevent infinite loops and cross-domain
    //! redirects that might be malicious.
    //!
    //! # Arguments
    //!
    //! * `url` - The shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL after following
    //! redirects, or `Err(Error)` if the expansion fails.
    //!
    //! # Behavior
    //!
    //! - Uses a custom redirect policy that stops when domains change
    //! - Follows standard HTTP 3xx redirects automatically
    //! - Returns the final URL from the last response
    //! - Handles network errors and timeouts appropriately
    let custom = custom_redirect_policy();
    ready(get_client_builder(timeout).redirect(custom).build())
        .and_then(|client| async move { client.get(url).send().await })
        .map_ok(|response| response.url().as_str().into())
        .err_into()
        .await
}

pub(crate) async fn unshort_with_browser_headers(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs using browser-like headers and full redirect following.
    //!
    //! This function mimics a browser request with proper headers to ensure
    //! that services like 2cm.es return proper redirects.
    //!
    //! # Arguments
    //!
    //! * `url` - The shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL after following
    //! all redirects, or `Err(Error)` if the expansion fails.
    //!
    //! # Behavior
    //!
    //! - Uses browser-like headers (Accept, Accept-Language, etc.)
    //! - Uses default redirect policy (allows cross-domain redirects)
    //! - Follows standard HTTP 3xx redirects automatically
    ready(get_client_builder(timeout).build())
        .and_then(|client| async move {
            client
                .get(url)
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
                .header("Accept-Language", "en-US,en;q=0.5")
                .header("Accept-Encoding", "gzip, deflate, br")
                .header("DNT", "1")
                .header("Connection", "keep-alive")
                .header("Upgrade-Insecure-Requests", "1")
                .send()
                .await
        })
        .map_ok(|response| response.url().as_str().into())
        .err_into()
        .await
}

pub(crate) async fn unshort_with_curl_ua(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs using curl user agent and full redirect following.
    //!
    //! This function uses curl's user agent string to ensure that services
    //! like t.co return proper HTTP redirects instead of HTML pages.
    //!
    //! # Arguments
    //!
    //! * `url` - The shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL after following
    //! all redirects, or `Err(Error)` if the expansion fails.
    //!
    //! # Behavior
    //!
    //! - Uses custom user agent string
    //! - Uses default redirect policy (allows cross-domain redirects)
    //! - Follows standard HTTP 3xx redirects automatically
    ready(get_client_builder(timeout)
        .user_agent("URLEXPANDER/0.3")
        .build())
        .and_then(|client| async move { client.get(url).send().await })
        .map_ok(|response| response.url().as_str().into())
        .err_into()
        .await
}
