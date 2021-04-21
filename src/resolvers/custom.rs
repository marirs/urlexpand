// Custom Resolver
use core::time::Duration;
use super::{build_client, from_re_pattern};

pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let client = match build_client(timeout) {
        Some(c) => c,
        None => return None
    };
    let response = client.get(url).send().ok().unwrap();
    let status = response.status();
    if status == 200 {

        let text = match response.text() {
            Ok(t) => t,
            _ => return None
        };

        return from_re_pattern(&text, "URL=(.*)\">")
    }

    None
}