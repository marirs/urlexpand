// All sites that performs Meta Refresh
use super::{from_re, from_url};
use std::time::Duration;

/// URL Expander for Shorten links that uses Meta Refresh to redirect
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    from_url(url, timeout).and_then(|html| from_re(&html, "URL=([^\"]*)"))
}
