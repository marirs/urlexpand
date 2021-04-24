// All sites that performs Meta Refresh
use std::time::Duration;
use super::{from_url, from_re_pattern};

/// URL Expander for Shorten links that uses Meta Refresh to redirect
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let html = match from_url(url, timeout) {
        Some(page) => page,
        None => return None,
    };

    from_re_pattern(&html, "URL=([^\"]*)")
}