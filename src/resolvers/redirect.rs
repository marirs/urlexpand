// Shortner services that Redirects
use super::{from_re, get_client_builder};
use std::time::Duration;

use futures::future::{ready, TryFutureExt};

use crate::{Error, Result};

static RE_PATTERNS: [&str; 6] = [
    r#"Here is the URL which you want to visit:<br><br>\n<a href="([^">]*)"#, // rlu.ru
    r#"window.open\(["']([^'"\)]*)"#, // redirects using window.open
    r#"window.location[= '"]*([^'"]*)"#, // redirects using window.location
    r#"target='_blank'>([^<]*)"#,     // nowlinks.net
    r#""redirecturl" href="(.*)">"#,  // tinyurl.com
    r#"src=['"]([^"']*)" scrolling"#, // vzturl.com
];

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs from shorteners that use JavaScript-based redirects.
    //!
    //! This resolver handles shorteners that don't use standard HTTP redirects
    //! but instead embed redirect information in JavaScript code within HTML pages.
    //! It searches for multiple common JavaScript redirect patterns.
    //!
    //! # Arguments
    //!
    //! * `url` - The shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the extracted destination URL on success,
    //! or `Err(Error)` if no redirect pattern is found.
    //!
    //! # Supported Patterns
    //!
    //! - `window.open()` calls
    //! - `window.location` assignments
    //! - HTML links with specific attributes
    //! - Custom redirect URL parameters
    //!
    //! # Behavior
    //!
    //! - Fetches the HTML content of the short URL
    //! - Searches for multiple JavaScript redirect patterns
    //! - Returns the first matching URL found
    ready(get_client_builder(timeout).build())
        .and_then(|client| async move { client.get(url).send().await })
        .and_then(|response| async move { response.text().await })
        .err_into()
        .and_then(|text| ready(from_re(&text, &RE_PATTERNS.join("|")).ok_or(Error::NoString)))
        .await
}
