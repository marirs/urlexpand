//! # urlexpand
//!
//! A small library for expanding (“unshortening”) shortened URLs into their final destination.
//!
//! The crate is designed around **resolver modules**, where each resolver knows how to expand
//! one (or a family) of shortener services—especially the ones that don’t rely purely on HTTP 3xx
//! redirects and instead use HTML/JS-based redirect pages.
//!
//! ## Goals
//!
//! - **Fast, reliable expansion** for common shorteners
//! - **Extensible** resolver structure (add a new module for a new shortener)
//! - **Non-JS resolution** (no headless browser) using a mix of redirect following + parsing + API lookups
//! - Consistent `Result<T>` / `Error` handling across resolvers
//!
//! ## How it works (high level)
//!
//! 1. The caller provides a URL (potentially shortened).
//! 2. The library picks a resolver (or tries several in order).
//! 3. The resolver expands the URL using one of these strategies:
//!    - **HTTP redirect following** (3xx chains)
//!    - **HTML pattern extraction** (regex-based “click-through” / meta / JS hints)
//!    - **Service API lookup** (when the browser normally uses JS to fetch the destination)
//! 4. The final URL is returned as a `String`.
//!
//! ## Module layout
//!
//! A common structure looks like this:
//!
//! - `src/lib.rs`
//!   - exports `Result` and `Error`
//!   - exports the public expansion API (if you have one)
//! - `src/error.rs`
//!   - defines `Error` and error conversions (e.g. `From<reqwest::Error>`)
//! - `src/resolvers/`
//!   - each file is a shortener-specific resolver (e.g. `tinyurl.rs`, `urlshortdev.rs`, etc.)
//! - `src/resolvers/mod.rs`
//!   - re-exports resolver functions and common helper utilities
//!
//! ## Common helper utilities
//!
//! Many resolver modules share helpers such as:
//!
//! - `get_client_builder(timeout)` — returns a configured `reqwest::ClientBuilder`
//! - `from_re(text, pattern)` — returns the first capture group match as `Option<String>`
//!
//! These helpers keep each resolver tiny and consistent.
//!
//! ## Error handling model
//!
//! Resolvers generally return:
//!
//! - `Ok(final_url)` on success
//! - `Err(Error::NoString)` when a redirect page/API response doesn’t contain a destination URL
//! - `Err(Error::...)` for network/HTTP/parse errors
//!
//! To make resolver modules ergonomic, it’s recommended that `Error` implements:
//!
//! - `From<reqwest::Error>`
//! - (optionally) `From<std::io::Error>` or other error conversions you use
//!
//! That lets resolvers freely use `?` or `.map_err(Error::from)`.
//!
//! ## Timeouts and redirect limits
//!
//! Timeouts are typically passed into each resolver (`Option<Duration>`) and applied via the shared
//! HTTP client builder. Redirect limits should also be configured in one place (your builder) so all
//! resolvers behave consistently.
//!
//! ## Adding a new resolver
//!
//! 1. Create `src/resolvers/<service>.rs`
//! 2. Implement:
//!
//! ```ignore
//! pub(crate) async fn unshort(url: &str, timeout: Option<std::time::Duration>) -> crate::Result<String> {
//!     // resolve & return final URL
//! }
//! ```
//!
//! 3. Re-export it from `src/resolvers/mod.rs`
//! 4. Add it to your dispatcher/registry if you have one (e.g., “try resolvers in order”).
//!
//! ### Resolver style guideline
//!
//! Keep resolvers small and focused:
//!
//! - follow redirects first
//! - if the service stops on a non-redirect “intermediate page”, use either:
//!   - regex extraction (`from_re`) or
//!   - a small API call if the browser normally uses JS
//!
//! ## Example usage (library consumer)
//!
//! If your crate exposes a high-level function (recommended), usage might look like:
//!
//! ```ignore
//! let final_url = urlexpand::expand("https://l1nq.com/yzjVi").await?;
//! println!("{final_url}");
//! ```
//!
//! Or if you call resolvers directly:
//!
//! ```ignore
//! let final_url = urlexpand::resolvers::urlshortdev::unshort("https://l1nq.com/yzjVi", None).await?;
//! ```
//!
//! ## Testing
//!
//! For deterministic tests, consider:
//!
//! - unit testing regex extraction helpers (`from_re`) with fixed strings
//! - using a mock HTTP server (or recorded fixtures) for network calls
//! - keeping “live” integration tests behind a feature flag, since shortener behavior can change
//!
//! ## Security considerations
//!
//! Expanding URLs can lead to untrusted destinations. Consider optional safeguards:
//!
//! - maximum redirect depth
//! - domain allow/deny lists
//! - blocking private IP ranges (SSRF protection) if this runs server-side
//! - request method restrictions (typically GET only)
//! - size limits for downloaded bodies when parsing HTML
use std::time::Duration;
use url::{ParseError, Url};

mod error;
mod resolvers;

mod services;
use services::{which_service, SERVICES};

#[cfg(test)]
mod tests;

pub type Error = error::Error;
pub type Result<T> = std::result::Result<T, Error>;

use futures::future::{ready, TryFutureExt};

pub fn is_shortened(url: &str) -> bool {
    //! Check to see if a given url is a shortened url
    //! ## Example
    //! ```rust
    //! use urlexpand::is_shortened;
    //!
    //! let url = "https://bit.ly/id";
    //! assert!(is_shortened(url));
    //! ```
    SERVICES.iter().any(|x| url.contains(x))
}

#[cfg(feature = "blocking")]
pub fn unshorten_blocking(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! UnShorten a shortened URL
    //! ## Example
    //! ```ignore
    //!  use std::time::Duration;
    //!  use urlexpand::unshorten_blocking;
    //!
    //!  let url = "https://bit.ly/3alqLKi";
    //!  assert!(unshorten_blocking(url, Some(Duration::from_secs(10))).await.is_some());   // with timeout
    //!  assert!(unshorten_blocking(url, None).await.is_some());    // without timeout
    //! ```
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(unshorten(url, timeout))
}

pub async fn unshorten(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! UnShorten a shortened URL
    //! ## Example
    //! ```ignore
    //!  use std::time::Duration;
    //!  use urlexpand::unshorten;
    //!
    //!  let url = "https://bit.ly/3alqLKi";
    //!  assert!(unshorten(url, Some(Duration::from_secs(10))).await.is_ok());   // with timeout
    //!  assert!(unshorten(url, None).await.is_ok());    // without timeout
    //! ```
    // Check to make sure url is valid
    ready(validate(url).ok_or(Error::NoString))
        .and_then(|validated_url| async move {
            let service = which_service(&validated_url).ok_or(Error::NoString)?;

            match service {
                // Adfly Resolver
                "adf.ly" | "atominik.com" | "fumacrom.com" | "intamema.com" | "j.gs" | "q.gs" => {
                    resolvers::adfly::unshort(&validated_url, timeout).await
                }

                // Redirect Resolvers
                "gns.io" | "ity.im" | "ldn.im" | "nowlinks.net" | "rlu.ru" | "tinyurl.com"
                | "tr.im" | "u.to" | "vzturl.com" => {
                    resolvers::redirect::unshort(&validated_url, timeout).await
                }

                // Meta Refresh Resolvers
                "cutt.us" | "soo.gd" => resolvers::refresh::unshort(&validated_url, timeout).await,

                // Specific Resolvers
                "adfoc.us" => resolvers::adfocus::unshort(&validated_url, timeout).await,
                "l1nq.com" | "sl1nk.com" => resolvers::urlshortdev::unshort(&validated_url, timeout).await,
                "lnkd.in" => resolvers::linkedin::unshort(&validated_url, timeout).await,
                "shorturl.at" => resolvers::shorturl::unshort(&validated_url, timeout).await,
                "surl.li" => resolvers::surlli::unshort(&validated_url, timeout).await,

                // Generic Resolvers
                _ => resolvers::generic::unshort(&validated_url, timeout).await,
            }
        })
        .await
}

/// Validate & return a clean URL
fn validate(u: &str) -> Option<String> {
    let parts = match Url::parse(u) {
        Ok(p) => p,
        Err(e) => match e {
            ParseError::RelativeUrlWithoutBase => {
                let new_url = format!("https://{}", u);
                match Url::parse(&new_url) {
                    Ok(p) => p,
                    Err(_) => return None,
                }
            }
            _ => return None,
        },
    };

    parts
        .domain()
        .and_then(|domain| is_shortened(domain).then(|| parts.as_str().into()))
}
