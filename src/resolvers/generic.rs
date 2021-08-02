// Generic Resolver
use std::time::Duration;

use super::{custom_redirect_policy, get_client_builder};

use futures::future::{ready, TryFutureExt};

use crate::Result;

/// Generic URL Expander
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    let custom = custom_redirect_policy();
    ready(get_client_builder(timeout).redirect(custom).build())
        .and_then(|client| async move { client.get(url).send().await })
        .map_ok(|response| response.url().as_str().into())
        .err_into()
        .await
}
