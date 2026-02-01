// adfoc.us shortening service
use std::time::Duration;

use super::from_url_not_200;

use futures::future::{ready, TryFutureExt};

use crate::{Error, Result};

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs shortened by Adfoc.us.
    //!
    //! This resolver handles Adfoc.us's JavaScript-based redirect mechanism
    //! which embeds the destination URL in a JavaScript variable rather than
    //! using standard HTTP redirects.
    //!
    //! # Arguments
    //!
    //! * `url` - The Adfoc.us shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL on success,
    //! or `Err(Error)` if the URL cannot be expanded.
    //!
    //! # Behavior
    //!
    //! - Fetches the HTML content of the short URL (expecting non-200 status)
    //! - Extracts the click_url parameter from JavaScript in the page
    //! - Returns the extracted URL
    from_url_not_200(url, timeout)
        .and_then(|html| {
            ready(
                html.split("click_url = \"")
                    .nth(1)
                    .and_then(|r| r.split("\";").next())
                    .map(Into::into)
                    .ok_or(Error::NoString),
            )
        })
        .await
}
