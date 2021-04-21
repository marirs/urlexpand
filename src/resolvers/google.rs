//goo.gl
use core::time::Duration;
use super::build_client;

/// goo.gl expander
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let client = match build_client(timeout) {
        Some(c) => c,
        None => return None
    };
    let response = client.get(url).send().ok().unwrap();
    let status = response.status();
    println!("{:?}", response);
    None
}