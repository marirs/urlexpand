// Generic Resolver
use core::time::Duration;
use reqwest::redirect::Policy;

use super::get_client_builder;

/// Generic URL Expander
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let custom = Policy::custom(|attempt| {
        let n_attempt = attempt.previous().len();
        if attempt.previous()[0].host() != attempt.previous()[n_attempt - 1].host() {
            attempt.stop()
        } else {
            attempt.follow()
        }
    });

    let client = match get_client_builder(timeout).redirect(custom).build() {
        Ok(c) => c,
        Err(_) => return None,
    };

    let response = client.get(url).send().ok().unwrap();
    Some(response.url().to_string())
}
