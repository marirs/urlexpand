//! Resolver for links shortened via urlshort.dev / l1nq.com.
//!
//! This module expands short URLs that ultimately rely on the
//! `encurtador.dev` redirect flow.
//!
//! ## How the resolution works
//!
//! These short links typically follow a two-stage process:
//!
//! 1. **Standard HTTP redirects**
//!    The initial short link (for example `https://l1nq.com/abc123`) performs
//!    normal HTTP 3xx redirects. `reqwest` follows these automatically.
//!
//! 2. **JavaScript-based redirect page**
//!    Some links stop at a URL like:
//!
//!    `https://www.encurtador.dev/redirecionamento/<CODE>`
//!
//!    This page does *not* return an HTTP redirect. Instead, a browser would
//!    execute JavaScript to retrieve the final destination from Encurtador’s
//!    API.
//!
//! 3. **API resolution (server-side)**
//!    To replicate the browser behavior without executing JavaScript, this
//!    resolver calls:
//!
//!    `https://dr-api.encurtador.dev/encurtamentos/<CODE>`
//!
//!    The API returns JSON containing the real destination URL in the `url`
//!    field. That value is returned as the fully expanded link.
//!
//! ## Behavior
//!
//! - Follows normal HTTP redirects automatically
//! - Detects Encurtador “redirecionamento” pages
//! - Resolves the final destination using Encurtador’s public API
//! - Returns the final URL as a `String`
//!
//! ## Errors
//!
//! This resolver returns `Error::NoString` if:
//! - The Encurtador API response does not contain a valid `url` field
//! - The redirect page does not match the expected format
//!
//! Network and HTTP errors are propagated via `Error` conversions from
//! `reqwest::Error`.
//!
//! ## Notes
//!
//! - Password-protected or expired links may not resolve via the API.
//! - The resolver does not execute JavaScript; it relies solely on HTTP and API calls.
//! - Redirect limits and timeouts are controlled by the shared HTTP client builder.
use super::{from_re, get_client_builder};
use std::time::Duration;

use futures::future::{ready, TryFutureExt};
use serde::Deserialize;

use crate::{Error, Result};

static ENCURTADOR_CODE_RE: &str =
    r#"https?://(?:www\.)?encurtador\.dev/redirecionamento/([A-Za-z0-9]+)"#;

#[derive(Debug, Deserialize)]
struct DrApiResp {
    url: Option<String>,
}

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    ready(get_client_builder(timeout).build())
        .map_err(Error::from)
        .and_then(|client| async move {
            let resp = client.get(url).send().await.map_err(Error::from)?;
            let final_url = resp.url().to_string();

            if let Some(code) = from_re(&final_url, ENCURTADOR_CODE_RE) {
                let api_url = format!("https://dr-api.encurtador.dev/encurtamentos/{}", code);

                let api_resp: DrApiResp = client
                    .get(api_url)
                    .header(reqwest::header::ACCEPT, "application/json,*/*")
                    .send()
                    .await
                    .map_err(Error::from)?
                    .error_for_status()
                    .map_err(Error::from)?
                    .json()
                    .await
                    .map_err(Error::from)?;

                return api_resp
                    .url
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().to_string())
                    .ok_or(Error::NoString);
            }

            Ok(final_url)
        })
        .await
}
