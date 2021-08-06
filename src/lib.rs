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
    //! fn main() {
    //!     let url = "https://bit.ly/id";
    //!     assert!(is_shortened(url));
    //! }
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
                "shorturl.at" => resolvers::shorturl::unshort(&validated_url, timeout).await,

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
