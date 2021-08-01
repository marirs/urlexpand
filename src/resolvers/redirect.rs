// Shortner services that Redirects
use super::{from_re, get_client_builder};
use std::time::Duration;

use futures::future::{ready, TryFutureExt};

use crate::{Error, Result};

static RE_PATTERNS: [&str; 6] = [
    r#"Here is the URL which you want to visit:<br><br>\n<a href="([^">]*)"#, // rlu.ru
    r#"window.open\(["']([^'"\)]*)"#, // redirects using window.open
    r#"window.location[= '"]*([^'"]*)"#, // redirects using window.location
    r#"target='_blank'>([^<]*)"#,     // nowlinks.net
    r#""redirecturl" href="(.*)">"#,  // tinyurl.com
    r#"src=['"]([^"']*)" scrolling"#, // vzturl.com
];

/// Shortner services that employ different Redirect mechanisms
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    ready(get_client_builder(timeout).build())
        .and_then(|client| async move { client.get(url).send().await })
        .and_then(|response| async move { response.text().await })
        .err_into()
        .and_then(|text| ready(from_re(&text, &RE_PATTERNS.join("|")).ok_or(Error::NoString)))
        .await
}
