// adf.ly and its associated domains
use std::{
    str::from_utf8,
    time::Duration,
    collections::VecDeque,
};
use percent_encoding::percent_decode_str;
use super::from_url;

/// Decode the YSMM variable value to fetch the dest url
fn decode_ysmm(ysmm: &str) -> Option<String> {
    let mut data = VecDeque::<char>::new();

    for c in ysmm.chars().collect::<Vec<_>>().chunks(2) {
        data.push_back(c[0]);
        data.push_front(c[1]);
    }

    data.rotate_left(data.len() / 2);

    let mut numbers: Vec<(usize, i32)> = Vec::new();
    for (j, val) in data.iter().enumerate() {
        if let Ok(val_parsed) = val.to_string().trim().parse() {
            numbers.push((j, val_parsed));
        }
    }
    for items in numbers.chunks(2) {
        if let [x, y] = items {
            let xor = x.1 ^ y.1;
            if xor < 10 {
                data[x.0] = xor.to_string().parse().unwrap();
            }
        }
    }

    let buf = base64::decode(data.drain(..).collect::<String>()).unwrap();
    match from_utf8(&buf) {
        Ok(v) => v[16..v.len() - 16].split("dest=").nth(1).map(|url| {
            percent_decode_str(url)
                .decode_utf8_lossy()
                .into()
        }),
        Err(_) => return None // Invalid UTF-8 sequence,
    }
}

/// URL Expander for ADF.LY and its associated shortners
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let html = match from_url(url, timeout) {
        Some(t) => t,
        None => return None
    };

    let ysmm = match html.split("ysmm = '").nth(1) {
        Some(r) => {
            match r.splitn(2, "';").next() {
                Some(t) => t,
                None => return None
            }
        },
        None => return None
    };

    decode_ysmm(ysmm)
}