//goo.gl
use super::build_client_builder;
use core::time::Duration;

use reqwest::redirect::Policy;

/// goo.gl expander
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let custom = Policy::custom(|attempt| {
        let n_attempt = attempt.previous().len();
        if attempt.previous()[0].host() != attempt.previous()[n_attempt - 1].host() {
            attempt.stop()
        } else {
            attempt.follow()
        }
    });

    let client = match build_client_builder(timeout).redirect(custom).build() {
        Ok(c) => c,
        Err(_) => return None,
    };
    let response = client.get(url).send().ok().unwrap();
    // let status = response.status();
    // println!("{:?}", response);
    Some(response.url().to_string())
}
