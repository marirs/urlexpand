// NOWLINKS.NET Shortner Service
use std::time::Duration;
use super::{from_url, from_re_pattern};

/// URL Expander for Now Links
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let html = match from_url(url, timeout) {
        Some(page) => page,
        None => return None,
    };

    return from_re_pattern(
        &html,
        "target='_blank'>([^<]*)"
    )
}