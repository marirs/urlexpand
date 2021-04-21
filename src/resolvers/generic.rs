// Generic Resolver
use super::build_client_builder;
use core::time::Duration;

/// Generic URL Expander
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let client = match build_client_builder(timeout).build() {
        Ok(c) => c,
        Err(_) => return None,
    };
    let response = match client.head(url).send() {
        Ok(r) => r,
        _ => return None,
    };
    let status = response.status();

    if status == 307 || status == 301 || status == 302 {
        // If Redirect is found
        let x = match response.headers().get("Location") {
            Some(x) => x,
            None => return None,
        };
        let s: String = match String::from_utf8(Vec::from(x.as_bytes())) {
            Ok(x) => x,
            Err(_) => {
                return None;
            }
        };
        return Some(s);
    } else if status == 200 {
        // If Http Ok
        let url = response.url();
        let scheme = url.scheme();
        let host = match url.host_str() {
            Some(h) => h,
            _ => "",
        };
        let port = match url.port() {
            Some(p) => p,
            _ => 0,
        };
        let path = url.path();
        let query_string = match url.query() {
            Some(q) => q,
            _ => "",
        };
        let urlexpanded = if port == 0 {
            format!("{}://{}{}&{}", scheme, host, path, query_string)
        } else {
            format!("{}://{}:{}{}&{}", scheme, host, port, path, query_string)
        };
        let urlexpanded = urlexpanded.trim_end_matches("&").to_string();
        return Some(urlexpanded);
    }

    None
}
