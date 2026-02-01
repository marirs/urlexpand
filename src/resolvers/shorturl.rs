// ShortURL.AT service
use super::{custom_redirect_policy, get_client_builder};
use std::time::Duration;

use futures::future::{ready, TryFutureExt};

use crate::{Error, Result};

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs shortened by shorturl.at.
    //!
    //! This resolver handles shorturl.at's specific redirect mechanism
    //! which uses HTTP HEAD requests and Location headers rather than
    //! standard GET requests or JavaScript redirects.
    //!
    //! # Arguments
    //!
    //! * `url` - The shorturl.at shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the destination URL from the Location header,
    //! or `Err(Error)` if the Location header is missing or the request fails.
    //!
    //! # Behavior
    //!
    //! - Uses HTTP HEAD request (more efficient for redirects)
    //! - Applies custom redirect policy for safety
    //! - Extracts destination URL from Location header
    //! - Handles network errors appropriately
    let custom = custom_redirect_policy();

    ready(get_client_builder(timeout).redirect(custom).build())
        .and_then(|client| async move { client.head(url).send().await })
        .err_into()
        .and_then(|response| {
            ready(
                response
                    .headers()
                    .get("location")
                    .ok_or(Error::NoString)
                    .and_then(|hv| Ok(hv.to_str()?.into())),
            )
        })
        .await
}
