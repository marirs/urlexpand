// adfoc.us shortening service
use std::time::Duration;

use super::from_url;

/// URL Expander for ADFOC.US
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let html = match from_url(url, timeout) {
        Some(t) => t,
        None => return None
    };

    let click_url = match html.split("click_url = \"").nth(1) {
        Some(r) => {
            match r.splitn(2, "\";").next() {
                Some(t) => t,
                None => return None
            }
        },
        None => return None
    };

    Some(click_url.to_string())
}