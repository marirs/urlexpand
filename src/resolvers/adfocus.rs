// adfoc.us shortening service
use std::time::Duration;

use super::from_url_not_200;

use futures::future::{ready, TryFutureExt};

use crate::{Error, Result};

/// URL Expander for ADFOC.US
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    from_url_not_200(url, timeout)
        .and_then(|html| {
            ready(
                html.split("click_url = \"")
                    .nth(1)
                    .and_then(|r| r.splitn(2, "\";").next())
                    .map(Into::into)
                    .ok_or(Error::NoString),
            )
        })
        .await
}
