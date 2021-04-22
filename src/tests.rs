use super::unshorten;

#[test]
fn test_bit_ly() {
    let url = "https://bit.ly/3alqLKi";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_b_link() {
    let url = "https://b.link/cx2x2l";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("http://www.google.com/".to_string()));
}

#[test]
fn test_buff_ly() {
    let url = "https://buff.ly/1GYcFvQ";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://blog.bufferapp.com/url-shorteners?"));
}

#[test]
fn test_chollo_to() {
    let url = "https://chollo.to/s1q4u";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.chollometro.com/ofertas/xiaomi-mi-band-6"));
}

#[test]
fn test_cutt_ly() {
    let url = "https://cutt.ly/tvDqE79";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_goo_gl() {
    let url = "https://goo.gl/cvSjeY";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url
        .unwrap()
        .starts_with("http://m.axisbank.com/"));
}

#[test]
fn test_gns_io() {
    let url = "https://gns.io/1RQl2";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://google.com/"));
}

#[test]
fn test_is_gd() {
    let url = "https://is.gd/EuvYes";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com"));
}

#[test]
fn test_iz4_short_gy() {
    let url = "https://iz4.short.gy/mr7KcJ";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com"));
}

#[test]
fn test_split_to() {
    let url = "https://split.to/V1ZhFut";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://google.com"));
}

#[test]
fn test_ldn_im() {
    let url = "http://ldn.im/1pNey";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://google.com/"));
}

#[test]
fn test_ow_ly() {
    let url = "http://ow.ly/j9qh7";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("http://t.co/cAcQV4QTOS".to_string()));
}

#[test]
fn test_rebrand_ly() {
    let url = "https://rebrand.ly/dp8cuo0";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_rotf_lol() {
    let url = "https://rotf.lol/4scu3nzz";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://google.com/".to_string()));
}

#[test]
fn test_t_co() {
    let url = "https://t.co/bYeHhy9kAU";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(
        expanded_url,
        Some("https://www.youtube.com/watch?v=x6QZn9xiuOE".to_string())
    );
}

#[test]
fn test_tiny_cc() {
    let url = "https://tiny.cc/5ocwtz";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_tinurl_com() {
    let url = "https://tinyurl.com/2j582c6a";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://google.com".to_string()));
}

#[test]
fn test_tiny_one() {
    let url = "https://tiny.one/f94uhh4x";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://google.com/".to_string()));
}

#[test]
fn test_tny_sh() {
    let url = "https://tny.sh/5C3X9Ss";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("http://google.com/".to_string()));
}

#[test]
fn test_tr_im() {
    let url = "https://tr.im/1iMz2";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://google.com/"));
}
