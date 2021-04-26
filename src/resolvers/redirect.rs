// Shortner services that Redirects
use std::time::Duration;
use super::{
    get_client_builder,
    from_re,
};

static RE_PATTERNS: [&str; 6] = [
    r#"Here is the URL which you want to visit:<br><br>\n<a href="([^">]*)"#, // rlu.ru
    r#"window.open\(["']([^'"\)]*)"#, // redirects using window.open
    r#"window.location[= '"]*([^'"]*)"#, // redirects using window.location
    r#"target='_blank'>([^<]*)"#, // nowlinks.net
    r#""redirecturl" href="(.*)">"#, // tinyurl.com
    r#"src=['"]([^"']*)" scrolling"#, // vzturl.com
];

/// Shortner services that employ different Redirect mechanisms
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let client = match get_client_builder(timeout).build() {
        Ok(c) => c,
        Err(_) => return None,
    };

    let response = match client.get(url).send() {
        Ok(r) => r,
        Err(_) => return None
    };

    let text = match response.text() {
        Ok(t) => t,
        _ => return None,
    };

    from_re(&text, &RE_PATTERNS.join("|"))
}
