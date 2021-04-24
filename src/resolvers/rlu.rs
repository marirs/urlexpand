// RLU.RU Shortner Service
use std::time::Duration;
use super::{from_url, from_re_pattern};

/// URL Expander for RLU Shortner Service
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let html = match from_url(url, timeout) {
        Some(page) => page,
        None => return None,
    };

    return from_re_pattern(
        &html,
        "Here is the URL which you want to visit:<br><br>\n<a href=\"([^\">]*)"
    )
}