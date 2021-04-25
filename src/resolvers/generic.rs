// Generic Resolver
use std::time::Duration;

use super::{custom_redirect_policy, get_client_builder};

/// Generic URL Expander
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let custom = custom_redirect_policy();
    let client = match get_client_builder(timeout).redirect(custom).build() {
        Ok(c) => c,
        Err(_) => return None,
    };

    let response = client.get(url).send().ok().unwrap();
    Some(response.url().to_string())
}
