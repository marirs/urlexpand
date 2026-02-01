// BROWSER Resolver
use crate::resolvers::generic;
use std::time::Duration;

use crate::Result;

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs using browser user agent for services that require browser-like requests.
    //!
    //! This resolver handles URLs that work with browser user agents but may not work
    //! with curl user agents (e.g., 2cm.es).
    //!
    //! # Arguments
    //!
    //! * `url` - The shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL on success,
    //! or `Err(Error)` if the URL cannot be expanded.
    //!
    //! # Behavior
    //!
    //! - Uses browser user agent string
    //! - Uses default redirect policy (allows cross-domain redirects)
    //! - Follows standard HTTP 3xx redirects automatically
    generic::unshort_with_browser_headers(url, timeout).await
}
