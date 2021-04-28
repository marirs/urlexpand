// adfoc.us shortening service
use std::time::Duration;

use super::from_url;

/// URL Expander for ADFOC.US
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    from_url(url, timeout).and_then(|html| {
        html.split("click_url = \"")
            .nth(1)
            .and_then(|r| r.splitn(2, "\";").next())
            .map(Into::into)
    })
}
