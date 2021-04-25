// ShortURL.AT service
use super::{custom_redirect_policy, get_client_builder};
use std::time::Duration;

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
        None => return None,
    };

    Some(dest)
}
