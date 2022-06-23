// SURL.LI Resolver
use crate::resolvers::{from_url, generic};
use futures::future::{ready, TryFutureExt};
use std::time::Duration;

use crate::{Error, Result};

/// Generic URL Expander
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    let expanded_url = generic::unshort(url, timeout).await?;
    Ok(
        if url.ends_with(expanded_url.split("//").last().unwrap_or_default()) {
            match get_from_html(url, timeout).await {
                Ok(u) => u,
                Err(_) => expanded_url,
            }
        } else {
            expanded_url
        },
    )
}

async fn get_from_html(url: &str, timeout: Option<Duration>) -> Result<String> {
    from_url(url, timeout)
        .and_then(|html| {
            ready(
                html.split("api.miniature.io/?url=")
                    .last()
                    .and_then(|r| r.split('"').next())
                    .map(|r| r.to_string())
                    .ok_or(Error::NoString),
            )
        })
        .await
}
