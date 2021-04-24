// adf.ly and its associated domains
use std::{
    str::from_utf8,
    time::Duration
};
use base64;

use super::from_url;

/// URL Decode
fn url_decode(url: &str) -> String {
    url
        .replace("%23", "#")
        .replace("%24", "$")
        .replace("%25", "%")
        .replace("%26", "&")
        .replace("%2B", "+")
        .replace("%2D", "-")
        .replace("%2F", "/")
        .replace("%3A", ":")
        .replace("%3D", "=")
        .replace("%3F", "?")
}

/// Decode the YSMM variable value to fetch the dest url
fn decode_ysmm(encoded: &str) -> Option<String> {
    let mut left: Vec<char> = Vec::new();
    let mut right: Vec<char> = Vec::new();

    let ysmm_subs = encoded.as_bytes()
        .chunks(2)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    for c in ysmm_subs {
        left.push(c.chars().nth(0).unwrap());
        right.insert(0, c.chars().nth(1).unwrap());
    }
    let mut encoded_text = [&left[..], &right[..]].concat();

    let mut nums: Vec<Vec<String>> = Vec::new();
    for (j, val) in encoded_text.clone().into_iter().enumerate() {
        if val.to_string().trim().parse::<usize>().is_ok() {
            nums.push(vec![j.to_string(), val.to_string()]);
        }
    }
    for x in (0..nums.len()).step_by(2) {
        if (nums.len() - 1) >= (x+1) {
            let first_int = &nums[x][1].parse::<usize>().unwrap();
            let second_int = &nums[x+1][1].parse::<usize>().unwrap();
            let first_index = &nums[x][0].parse::<usize>().unwrap();
            let xor = first_int ^ second_int;
            if xor < 10 {
                let xor_char: char = xor.to_string().parse().unwrap();
                encoded_text[*first_index as usize] =  xor_char;
            }
        }
    }

    let encoded_uri_str: String = encoded_text.into_iter().collect();

    let buf = base64::decode(&encoded_uri_str).unwrap();
    let decoded = match from_utf8(&buf) {
        Ok(v) => &v[16..v.len()-16],
        Err(_) => return None // Invalid UTF-8 sequence
    };

    let dest_uri = match decoded.split("dest=").nth(1) {
        Some(u) => url_decode(u),
        None => return None
    };
    Some(dest_uri)
}

/// URL Expander for ADF.LY and its associated shortners
pub(crate) fn unshort(url: &str, timeout: Option<Duration>) -> Option<String> {
    let html = match from_url(url, timeout) {
        Some(t) => t,
        None => return None
    };

    let ysmm = match html.split("ysmm = '").nth(1) {
        Some(r) => {
            match r.splitn(2, "';").nth(0) {
                Some(t) => t,
                None => return None
            }
        },
        None => return None
    };

    decode_ysmm(ysmm)
}