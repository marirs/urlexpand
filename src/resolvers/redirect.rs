// Shortner services that Redirects
use super::{from_re, get_client_builder};
use std::time::Duration;

static RE_PATTERNS: [&str; 6] = [
    r#"Here is the URL which you want to visit:<br><br>\n<a href="([^">]*)"#, // rlu.ru
    r#"window.open\(["']([^'"\)]*)"#, // redirects using window.open
    r#"window.location[= '"]*([^'"]*)"#, // redirects using window.location
    r#"target='_blank'>([^<]*)"#,     // nowlinks.net
    r#""redirecturl" href="(.*)">"#,  // tinyurl.com
    r#"src=['"]([^"']*)" scrolling"#, // vzturl.com
];

/// Shortner services that employ different Redirect mechanisms
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    get_client_builder(timeout)
        .build()
        .and_then(|client| client.get(url).send())
        .and_then(|response| response.text())
        .ok()
        .and_then(|text| from_re(&text, &RE_PATTERNS.join("|")))
}
