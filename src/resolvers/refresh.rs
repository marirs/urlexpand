// All sites that performs Meta Refresh
use super::{from_re, from_url_not_200};
use std::time::Duration;

use futures::future::{ready, TryFutureExt};

use crate::{Error, Result};

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs from shorteners that use HTML meta refresh redirects.
    //!
    //! This resolver handles shorteners that use HTML meta refresh tags
    //! instead of HTTP redirects or JavaScript. The destination URL is
    //! embedded in a meta tag's URL attribute.
    //!
    //! # Arguments
    //!
    //! * `url` - The shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the extracted destination URL on success,
    //! or `Err(Error)` if no meta refresh tag is found.
    //!
    //! # Behavior
    //!
    //! - Fetches the HTML content (expecting non-200 status)
    //! - Searches for meta refresh tags with URL parameter
    //! - Extracts and returns the destination URL
    from_url_not_200(url, timeout)
        .and_then(|html| ready(from_re(&html, "URL=([^\"]*)").ok_or(Error::NoString)))
        .await
}
