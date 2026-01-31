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
use super::get_client_builder;
use std::time::Duration;

use futures::future::{ready, TryFutureExt};
use serde::Deserialize;
use url::Url;

use crate::{Error, Result, services::which_service};

#[derive(Debug, Deserialize)]
struct DrApiResp {
    url: Option<String>,
}

/// Extract slug from URL:
/// 1) If encurtador redirecionamento page → last segment after "redirecionamento"
/// 2) Otherwise, if URL belongs to a known shortener domain → last path segment
fn extract_slug(u: &str) -> Option<String> {
    let parsed = Url::parse(u).ok()?;
    let host = parsed.host_str()?.to_ascii_lowercase();

    let segments: Vec<&str> = parsed
        .path_segments()
        .map(|it| it.filter(|s| !s.is_empty()).collect())
        .unwrap_or_else(Vec::new);

    if segments.is_empty() {
        return None;
    }

    // Case 1: encurtador redirect landing page
    if host == "encurtador.dev" || host == "www.encurtador.dev" {
        if let Some(i) = segments.iter().position(|s| *s == "redirecionamento") {
            return segments.get(i + 1..)?.last().map(|s| (*s).to_string());
        }
        return None;
    }

    // Case 2: any known shortener domain from your SERVICES list
    if which_service(u).is_some() {
        return segments.last().map(|s| (*s).to_string());
    }

    None
}

/// Resolve slug via dr-api:
/// - If dr-api responds with redirect → return Location target
/// - If dr-api responds with JSON { url } → return that
async fn resolve_via_dr_api(
    client_no_redirect: &reqwest::Client,
    slug: &str,
) -> Result<String> {
    let api_url = format!("https://dr-api.encurtador.dev/encurtamentos/{}", slug);

    let resp = client_no_redirect
        .get(&api_url)
        .header(reqwest::header::ACCEPT, "application/json,*/*")
        .send()
        .await?;

    // Case A: redirect
    if resp.status().is_redirection() {
        if let Some(loc) = resp.headers().get(reqwest::header::LOCATION) {
            let s = loc.to_str().unwrap_or("").trim();
            if !s.is_empty() {
                return Ok(s.to_string());
            }
        }
    }

    // Case B: JSON
    if resp.status().is_success() {
        let data: DrApiResp = resp.json().await?;
        if let Some(u) = data.url {
            let u = u.trim().to_string();
            if !u.is_empty() {
                return Ok(u);
            }
        }
    }

    Err(Error::Reqwest("dr-api could not resolve slug".to_string()))
}

/// Resolver for urlshort.dev-style links
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    ready(get_client_builder(timeout).build())
        .map_err(Error::from)
        .and_then(|client| async move {
            // Client that does NOT auto-follow redirects (so we can read Location headers)
            let client_no_redirect = get_client_builder(timeout)
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .map_err(Error::from)?;

            // Step 1: follow redirects normally to see where we land
            let resp = client.get(url).send().await.map_err(Error::from)?;
            let final_url = resp.url().to_string();

            // Step 2: extract slug (prefer final_url, fallback to original)
            let slug = extract_slug(&final_url)
                .or_else(|| extract_slug(url))
                .ok_or(Error::NoString)?;

            // Step 3: resolve via dr-api
            let resolved = resolve_via_dr_api(&client_no_redirect, &slug)
                .await
                .map_err(Error::from)?;

            Ok(resolved)
        })
        .await
}
