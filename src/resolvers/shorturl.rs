// ShortURL.AT service
use std::time::Duration;
use super::{get_client_builder, custom_redirect_policy};

/// URL Expander for shorturl.at Shortner Service
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let custom = custom_redirect_policy();
    let client = match get_client_builder(timeout).redirect(custom).build() {
        Ok(c) => c,
        Err(_) => return None,
    };
    let response = client.head(url).send().ok().unwrap();
    let dest = match response.headers().get("location") {
        Some(u) => u.to_str().ok().unwrap().to_string(),
        None => return None
    };

    Some(dest)
}