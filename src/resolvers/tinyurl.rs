// TinyUrl.Com Service
use super::{get_client_builder, from_re_pattern};
use core::time::Duration;

pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let client = match get_client_builder(timeout).build() {
        Ok(c) => c,
        Err(_) => return None,
    };
    let response = client.get(url).send().ok().unwrap();
    let status = response.status();
    if status == 200 {
        let text = match response.text() {
            Ok(t) => t,
            _ => return None,
        };

        return from_re_pattern(&text, "\"redirecturl\" href=\"(.*)\">");
    }

    None
}
