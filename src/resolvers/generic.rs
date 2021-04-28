// Generic Resolver
use std::time::Duration;

use super::{custom_redirect_policy, get_client_builder};

/// Generic URL Expander
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let custom = custom_redirect_policy();
    get_client_builder(timeout)
        .redirect(custom)
        .build()
        .and_then(|client| client.get(url).send())
        .map(|response| response.url().as_str().into())
        .ok()
}
