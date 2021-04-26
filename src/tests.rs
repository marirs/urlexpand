use super::{
    is_shortened,
    unshorten,
    validate,
};

#[test]
fn test_validate() {
    assert!(validate("bit.ly").is_some());
    assert!(validate("https://bit.ly/").is_some());
    assert!(validate("bit").is_none());
    assert!(validate("https://bit").is_none());
    assert!(validate("google.com").is_none());
    assert!(validate("google").is_none());
}

#[test]
fn test_is_shortened() {
    let url = "https://bit.ly/3alqLKi";
    assert!(is_shortened(url));
    let url = "https://www.google.com";
    assert!(!is_shortened(url));
}

#[test]
fn test_adf_ly() {
    let url = "https://adf.ly/HmtTG";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("http://google.com".to_string()));
}

#[test]
fn test_adfoc_us() {
    let url = "http://adfoc.us/x1";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("http://google.com/".to_string()));
}

#[test]
fn test_amzn_to() {
    let url = "https://amzn.to/2SdesXo";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.amazon.com/gp/offer-listing/"));
}

#[test]
fn test_atominik_com() {
    let url = "https://atominik.com/2YTd";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().contains("weebly.com"));
}

#[test]
fn test_bit_ly() {
    let url = "https://bit.ly/3alqLKi";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_bit_do() {
    let url = "http://bit.do/fQy4h";
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
fn test_bn_gy() {
    let url = "https://bn.gy/x7xUl";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://google.com/".to_string()));
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
fn test_cli_re() {
    let url = "https://cli.re/wxbz38";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
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
fn test_cutt_us() {
    let url = "https://cutt.us/keYiy";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com".to_string()));
}

#[test]
fn test_db_tt() {
    let url = "https://db.tt/cchelp";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.dropbox.com/"));
}

#[test]
fn test_fb_me() {
    let url = "https://fb.me/mashable";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.facebook.com/mashable".to_string()));
}

#[test]
fn test_flip_it() {
    let url = "http://flip.it/3jbsWn";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.thefuturist.co/why-is-ethereum-the-future-of-finance/".to_string()));
}

#[test]
fn test_fumacrom_com() {
    let url = "https://fumacrom.com/1KP3";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com".to_string()));
}

#[test]
fn test_j_gs() {
    let url = "http://j.gs/AXr9";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://microsoft.com".to_string()));
}

#[test]
fn test_git_io() {
    let url = "https://git.io/JOiM6";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.github.com/marirs/urlexpand".to_string()));
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
fn test_hmm_rs() {
    let url = "http://hmm.rs/Hangfire.PostgreSql";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://github.com/"));
}

#[test]
fn test_hyperurl_co() {
    let url = "https://hyperurl.co/qicb73";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_intamema_com() {
    let url = "http://intamema.com/HjU";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("http://www.google.com"));
}

#[test]
fn test_is_gd() {
    let url = "https://is.gd/EuvYes";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com/"));
}

#[test]
fn test_ity_im() {
    let url = "http://ity.im/U8re4";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com/search?q=rust&"));
}

#[test]
fn test_iz4_short_gy() {
    let url = "https://iz4.short.gy/mr7KcJ";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com"));
}

#[test]
fn test_kutt_it() {
    let url = "https://kutt.it/jO2XmP";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_ldn_im() {
    let url = "http://ldn.im/1pNey";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://google.com/"));
}

#[test]
fn test_linklyhq_com() {
    let url = "https://l.linklyhq.com/l/QebZ";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com/"));
}

#[test]
fn test_mlz_la() {
    let url = "https://mzl.la/3eqJ565";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("http://support.mozilla.org/".to_string()));
}

#[test]
fn test_ow_ly() {
    let url = "http://ow.ly/j9qh7";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("http://t.co/cAcQV4QTOS".to_string()));
}

#[test]
fn test_q_gs() {
    let url = "http://q.gs/FNOHk";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com".to_string()));
}

#[test]
fn test_qr_ae() {
    let url = "http://qr.ae/7FQS9";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("http://www.quora.com"));
}

#[test]
fn test_rb_gy() {
    let url = "https://rb.gy/ciq6si";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://google.com/".to_string()));
}

#[test]
fn test_rebrand_ly() {
    let url = "https://rebrand.ly/dp8cuo0";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_rlu_ru() {
    let url = "https://rlu.ru/1B5";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com".to_string()));
}

#[test]
fn test_rotf_lol() {
    let url = "https://rotf.lol/4scu3nzz";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://google.com/".to_string()));
}

#[test]
fn test_s_coop() {
    let url = "https://s.coop/7oxn";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com/"));
}

#[test]
fn test_s_id() {
    let url = "https://s.id/A87Cn";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_shorturl_at() {
    let url = "https://shorturl.at/kmrEO";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com".to_string()));
}

#[test]
fn test_split_to() {
    let url = "https://split.to/V1ZhFut";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://google.com"));
}

#[test]
fn test_smc_sg() {
    let url = "https://smu.sg/4l4";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://google.com/"));
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
fn test_t_ly() {
    let url = "https://t.ly/2ESW";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_t2m_io() {
    let url = "https://t2m.io/SSQhKqJ2";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_tiny_cc() {
    let url = "https://tiny.cc/5ocwtz";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_tiny_pl() {
    let url = "https://tiny.pl/rsjgq";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com/search?q=rust&"));
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

#[test]
fn test_u_to() {
    let url = "https://u.to/P05FGw";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com/search?q=rust"));
}

#[test]
fn test_v_gd() {
    let url = "https://v.gd/6H6dYQ";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
}

#[test]
fn test_virg_in() {
    let url = "https://virg.in/9sj";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("https://www.virginactive.co.za/quest".to_string()));
}

#[test]
fn test_vzturl_com() {
    let url = "https://vzturl.com/bqd20";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.amazon.com/Sabrent-Thunderbolt-"));
}

#[test]
fn test_yourwish_es() {
    let url = "http://yourwish.es/oxgyc";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert!(expanded_url.unwrap().starts_with("https://www.amazon.com/Sabrent-Thunderbolt"));
}

#[test]
fn test_zpr_io() {
    let url = "http://zpr.io/nniJB";
    let expanded_url = unshorten(url, None);
    assert!(expanded_url.is_some());
    assert_eq!(expanded_url, Some("http://www.archiveteam.org/".to_string()));
}
