// CURL Resolver
use crate::resolvers::generic;
use std::time::Duration;

use crate::Result;

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs using curl user agent for services that require command-line tools.
    //!
    //! This resolver handles URLs that work with curl user agent but may not work
    //! with browser user agents (e.g., t.co, goo.gl).
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
    //! - Uses curl user agent string
    //! - Uses default redirect policy (allows cross-domain redirects)
    //! - Follows standard HTTP 3xx redirects automatically
    generic::unshort_with_curl_ua(url, timeout).await
}
