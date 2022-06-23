// All sites that performs Meta Refresh
use super::{from_re, from_url_not_200};
use std::time::Duration;

use futures::future::{ready, TryFutureExt};

use crate::{Error, Result};

/// URL Expander for Shorten links that uses Meta Refresh to redirect
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    from_url_not_200(url, timeout)
        .and_then(|html| ready(from_re(&html, "URL=([^\"]*)").ok_or(Error::NoString)))
        .await
}
