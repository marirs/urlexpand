// adf.ly and its associated domains
use super::from_url_not_200;
use base64::{engine::general_purpose, Engine as _};
use futures::future::{ready, TryFutureExt};
use percent_encoding::percent_decode_str;
use std::{collections::VecDeque, str::from_utf8, time::Duration};

use crate::{Error, Result};

fn decode_ysmm(ysmm: &str) -> Option<String> {
    //! Decodes the YSMM (Your Safe Money Maker) variable used by Adf.ly
    //! to obfuscate the final destination URL.
    //!
    //! This function implements the custom decoding algorithm used by Adf.ly
    //! to extract the real URL from the encoded YSMM parameter found in their
    //! redirect pages.
    //!
    //! # Arguments
    //!
    //! * `ysmm` - The encoded YSMM string from the Adf.ly page
    //!
    //! # Returns
    //!
    //! Returns `Some(String)` with the decoded destination URL if successful,
    //! or `None` if the decoding fails or the format is invalid.
    //!
    //! # Algorithm
    //!
    //! 1. Rearranges characters in a specific pattern
    //! 2. Rotates the character array
    //! 3. Performs XOR operations on numeric values
    //! 4. Base64 decodes the result
    //! 5. Extracts the final URL from the "dest=" parameter
    let mut data = VecDeque::<char>::new();

    for c in ysmm.chars().collect::<Vec<_>>().chunks(2) {
        data.push_back(c[0]);
        data.push_front(c[1]);
    }

    data.rotate_left(data.len() / 2);

    data.iter()
        .enumerate()
        .filter_map(|(j, val)| {
            val.to_string()
                .trim()
                .parse::<i32>()
                .map(|val_parsed| (j, val_parsed))
                .ok()
        })
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .for_each(|items| {
            if let [x, y] = items {
                let xor = x.1 ^ y.1;
                if xor < 10 {
                    data[x.0] = xor.to_string().parse().unwrap();
                }
            }
        });

    let buf = general_purpose::STANDARD
        .decode(data.drain(..).collect::<String>())
        .unwrap();

    from_utf8(&buf).ok().and_then(|v| {
        v[16..v.len() - 16]
            .split("dest=")
            .nth(1)
            .map(|url| percent_decode_str(url).decode_utf8_lossy().into())
    })
}

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs shortened by Adf.ly and its associated domains.
    //!
    //! This resolver handles Adf.ly's custom redirect mechanism which uses
    //! JavaScript-based redirection with an encoded YSMM parameter rather
    //! than standard HTTP redirects.
    //!
    //! # Arguments
    //!
    //! * `url` - The Adf.ly shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL on success,
    //! or `Err(Error)` if the URL cannot be expanded.
    //!
    //! # Behavior
    //!
    //! - Fetches the HTML content of the short URL (expecting non-200 status)
    //! - Extracts the YSMM parameter from JavaScript in the page
    //! - Decodes the YSMM parameter to reveal the final destination
    //! - Returns the decoded URL
    from_url_not_200(url, timeout)
        .and_then(|html| {
            ready(
                html.split("ysmm = '")
                    .nth(1)
                    .and_then(|r| r.split("';").next())
                    .and_then(decode_ysmm)
                    .ok_or(Error::NoString),
            )
        })
        .await
}
