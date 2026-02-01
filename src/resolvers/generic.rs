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
