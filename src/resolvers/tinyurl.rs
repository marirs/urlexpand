// TinyUrl.Com Service
use std::time::Duration;
use super::{from_url, from_re_pattern};

/// URL Expander for TinyURL Service
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let text = match from_url(url, timeout) {
        Some(page) => page,
        None => return None,
    };

    from_re_pattern(&text, "\"redirecturl\" href=\"(.*)\">")
}
