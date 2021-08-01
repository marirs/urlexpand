// adf.ly and its associated domains
use super::from_url;
use percent_encoding::percent_decode_str;
use std::{collections::VecDeque, str::from_utf8, time::Duration};

use futures::future::{ready, TryFutureExt};

use crate::{Error, Result};

/// Decode the YSMM variable value to fetch the dest url
fn decode_ysmm(ysmm: &str) -> Option<String> {
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

    let buf = base64::decode(data.drain(..).collect::<String>()).unwrap();

    from_utf8(&buf).ok().and_then(|v| {
        v[16..v.len() - 16]
            .split("dest=")
            .nth(1)
            .map(|url| percent_decode_str(url).decode_utf8_lossy().into())
    })
}

/// URL Expander for ADF.LY and its associated shortners
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    from_url(url, timeout)
        .and_then(|html| {
            ready(
                html.split("ysmm = '")
                    .nth(1)
                    .and_then(|r| r.splitn(2, "';").next())
                    .and_then(|ysmm| decode_ysmm(ysmm))
                    .ok_or(Error::NoString),
            )
        })
        .await
}
