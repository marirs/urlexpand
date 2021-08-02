// ShortURL.AT service
use super::{custom_redirect_policy, get_client_builder};
use std::time::Duration;

use futures::future::{ready, TryFutureExt};

use crate::{Error, Result};

/// URL Expander for shorturl.at Shortner Service
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    let custom = custom_redirect_policy();

    ready(get_client_builder(timeout).redirect(custom).build())
        .and_then(|client| async move { client.head(url).send().await })
        .err_into()
        .and_then(|response| {
            ready(
                response
                    .headers()
                    .get("location")
                    .ok_or(Error::NoString)
                    .and_then(|hv| Ok(hv.to_str()?.into())),
            )
        })
        .await
}
