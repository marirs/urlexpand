use std::time::Duration;
use url::{ParseError, Url};

mod resolvers;

mod services;
use services::{which_service, SERVICES};

#[cfg(test)]
mod tests;

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

pub fn unshorten(url: &str, timeout: Option<Duration>) -> Option<String> {
    //! UnShorten a shortened URL
    //! ## Example
    //! ```rust
    //! use std::time::Duration;
    //! use urlexpand::unshorten;
    //!
    //! fn main() {
    //!     let url = "https://bit.ly/3alqLKi";
    //!     assert!(unshorten(url, Some(Duration::from_secs(10))).is_some());   // with timeout
    //!     assert!(unshorten(url, None).is_some());    // without timeout
    //! }
    //! ```
    // Check to make sure url is valid
    validate(url).and_then(|validated_url| {
        which_service(&validated_url).and_then(|service| match service {
            // Adfly Resolver
            "adf.ly" | "atominik.com" | "fumacrom.com" | "intamema.com" | "j.gs" | "q.gs" => {
                resolvers::adfly::unshort(&validated_url, timeout)
            }

            // Redirect Resolvers
            "gns.io" | "ity.im" | "ldn.im" | "nowlinks.net" | "rlu.ru" | "tinyurl.com"
            | "tr.im" | "u.to" | "vzturl.com" => {
                resolvers::redirect::unshort(&validated_url, timeout)
            }

            // Meta Refresh Resolvers
            "cutt.us" | "soo.gd" => resolvers::refresh::unshort(&validated_url, timeout),

            // Specific Resolvers
            "adfoc.us" => resolvers::adfocus::unshort(&validated_url, timeout),
            "shorturl.at" => resolvers::shorturl::unshort(&validated_url, timeout),

            // Generic Resolvers
            _ => resolvers::generic::unshort(&validated_url, timeout),
        })
    })
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
