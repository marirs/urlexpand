// Shortner services that use window.open or window.location
use std::time::Duration;
use super::{
    get_client_builder,
    from_re_pattern,
};

/// Shortner services that use window.open or window.location
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
    if text.contains("window.open(\"") {
        from_re_pattern(&text, r#"window.open\("([^"\)]*)"#)
    } else if text.contains("window.location='") {
        from_re_pattern(&text, r#"window.location='(.*)';"#)
    } else {
        None
    }
}
