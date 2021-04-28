// ShortURL.AT service
use super::{custom_redirect_policy, get_client_builder};
use std::time::Duration;

/// URL Expander for shorturl.at Shortner Service
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let custom = custom_redirect_policy();

    get_client_builder(timeout)
        .redirect(custom)
        .build()
        .and_then(|client| client.head(url).send())
        .ok()
        .and_then(|response| {
            response
                .headers()
                .get("location")
                .and_then(|hv| hv.to_str().ok())
                .map(Into::into)
        })
}
