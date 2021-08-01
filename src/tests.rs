use super::{is_shortened, unshorten, unshorten_blocking, validate};

use paste::paste;

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

macro_rules! test_shorten_link {
    ($t_name:ident, $s_url:expr,$e_url:expr) => {
        #[tokio::test]
        async fn $t_name() {
            let url = $s_url;
            let expanded_url = unshorten(url, None).await;
            assert!(expanded_url.is_ok());
            assert_eq!(expanded_url, Ok($e_url.to_string()));
        }

        // until std::concat_idents stablizes
        paste! {
            #[test]
            fn [<$t_name _blocking>]() {
                let url = $s_url;
                let expanded_url = unshorten_blocking(url, None);
                assert!(expanded_url.is_ok());
                assert_eq!(expanded_url, Ok($e_url.to_string()));
            }
        }
    };
}

test_shorten_link!(test_adf_ly, "https://adf.ly/HmtTG", "http://google.com");
test_shorten_link!(test_adfoc_us, "http://adfoc.us/x1", "http://google.com");
test_shorten_link!(test_amzn_to, "https://amzn.to/2SdesXo", "https://www.amazon.com/gp/offer-listing/");

#[tokio::test]
async fn test_atominik_com() {
    let url = "https://atominik.com/2YTd";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().contains("weebly.com"));
}

#[tokio::test]
async fn test_bit_ly() {
    let url = "https://bit.ly/3alqLKi";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_bit_do() {
    let url = "http://bit.do/fQy4h";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_b_link() {
    let url = "https://b.link/cx2x2l";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("http://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_bn_gy() {
    let url = "https://bn.gy/x7xUl";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://google.com/".to_string()));
}

#[tokio::test]
async fn test_buff_ly() {
    let url = "https://buff.ly/1GYcFvQ";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://blog.bufferapp.com/url-shorteners?"));
}

#[tokio::test]
async fn test_cli_re() {
    let url = "https://cli.re/wxbz38";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_chollo_to() {
    let url = "https://chollo.to/s1q4u";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.chollometro.com/ofertas/xiaomi-mi-band-6"));
}

#[tokio::test]
async fn test_cutt_ly() {
    let url = "https://cutt.ly/tvDqE79";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_cutt_us() {
    let url = "https://cutt.us/keYiy";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com".to_string()));
}

#[tokio::test]
async fn test_db_tt() {
    let url = "https://db.tt/cchelp";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.dropbox.com/"));
}

#[tokio::test]
async fn test_fb_me() {
    let url = "https://fb.me/mashable";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(
        expanded_url,
        Ok("https://www.facebook.com/mashable".to_string())
    );
}

#[tokio::test]
async fn test_fumacrom_com() {
    let url = "https://fumacrom.com/1KP3";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com".to_string()));
}

#[tokio::test]
async fn test_j_gs() {
    let url = "http://j.gs/AXr9";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://microsoft.com".to_string()));
}

#[tokio::test]
async fn test_git_io() {
    let url = "https://git.io/JOiM6";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(
        expanded_url,
        Ok("https://www.github.com/marirs/urlexpand".to_string())
    );
}

#[tokio::test]
async fn test_goo_gl() {
    let url = "https://goo.gl/cvSjeY";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("http://m.axisbank.com/"));
}

#[tokio::test]
async fn test_gns_io() {
    let url = "https://gns.io/1RQl2";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://google.com/"));
}

#[tokio::test]
async fn test_hmm_rs() {
    let url = "http://hmm.rs/Hangfire.PostgreSql";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://github.com/"));
}

#[tokio::test]
async fn test_hyperurl_co() {
    let url = "https://hyperurl.co/qicb73";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_intamema_com() {
    let url = "http://intamema.com/HjU";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("http://www.google.com"));
}

#[tokio::test]
async fn test_is_gd() {
    let url = "https://is.gd/EuvYes";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com/"));
}

#[tokio::test]
async fn test_ity_im() {
    let url = "http://ity.im/U8re4";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.google.com/search?q=rust&"));
}

#[tokio::test]
async fn test_iz4_short_gy() {
    let url = "https://iz4.short.gy/mr7KcJ";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com"));
}

#[tokio::test]
async fn test_kutt_it() {
    let url = "https://kutt.it/jO2XmP";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_ldn_im() {
    let url = "http://ldn.im/1pNey";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://google.com/"));
}

#[tokio::test]
async fn test_linklyhq_com() {
    let url = "https://l.linklyhq.com/l/QebZ";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com/"));
}

#[tokio::test]
async fn test_mlz_la() {
    let url = "https://mzl.la/3eqJ565";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("http://support.mozilla.org/".to_string()));
}

#[tokio::test]
async fn test_ow_ly() {
    let url = "http://ow.ly/j9qh7";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("http://t.co/cAcQV4QTOS".to_string()));
}

#[tokio::test]
async fn test_plu_sh() {
    let url = "https://plu.sh/xnwb8";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_q_gs() {
    let url = "http://q.gs/async fnOHk";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com".to_string()));
}

#[tokio::test]
async fn test_qr_ae() {
    let url = "http://qr.ae/7FQS9";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("http://www.quora.com"));
}

#[tokio::test]
async fn test_rb_gy() {
    let url = "https://rb.gy/ciq6si";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://google.com/".to_string()));
}

#[tokio::test]
async fn test_rebrand_ly() {
    let url = "https://rebrand.ly/dp8cuo0";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_rlu_ru() {
    let url = "https://rlu.ru/1B5";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com".to_string()));
}

#[tokio::test]
async fn test_rotf_lol() {
    let url = "https://rotf.lol/4scu3nzz";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://google.com/".to_string()));
}

#[tokio::test]
async fn test_s_coop() {
    let url = "https://s.coop/7oxn";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://www.google.com/"));
}

#[tokio::test]
async fn test_s_id() {
    let url = "https://s.id/A87Cn";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_shorturl_at() {
    let url = "https://shorturl.at/kmrEO";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com".to_string()));
}

#[tokio::test]
async fn test_split_to() {
    let url = "https://split.to/V1ZhFut";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://google.com"));
}

#[tokio::test]
async fn test_smc_sg() {
    let url = "https://smu.sg/4l4";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://google.com/"));
}

#[tokio::test]
async fn test_snip_ly() {
    let url = "snip.ly/soyummy-cookbook";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.soyummystore.com"));
}

#[tokio::test]
async fn test_t_co() {
    let url = "https://t.co/bYeHhy9kAU";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(
        expanded_url,
        Ok("https://www.youtube.com/watch?v=x6QZn9xiuOE".to_string())
    );
}

#[tokio::test]
async fn test_t_ly() {
    let url = "https://t.ly/2ESW";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_t2m_io() {
    let url = "https://t2m.io/SSQhKqJ2";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_tiny_cc() {
    let url = "https://tiny.cc/5ocwtz";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_tiny_pl() {
    let url = "https://tiny.pl/rsjgq";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.google.com/search?q=rust&"));
}

#[tokio::test]
async fn test_tinurl_com() {
    let url = "https://tinyurl.com/2j582c6a";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://google.com".to_string()));
}

#[tokio::test]
async fn test_tiny_one() {
    let url = "https://tiny.one/f94uhh4x";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://google.com/".to_string()));
}

#[tokio::test]
async fn test_tny_sh() {
    let url = "https://tny.sh/5C3X9Ss";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("http://google.com/".to_string()));
}

#[tokio::test]
async fn test_tr_im() {
    let url = "https://tr.im/1iMz2";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url.unwrap().starts_with("https://google.com/"));
}

#[tokio::test]
async fn test_trib_al() {
    let url = "https://trib.al/YKNecc2";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://mashable.com/article"));
}

#[tokio::test]
async fn test_u_to() {
    let url = "https://u.to/P05FGw";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.google.com/search?q=rust"));
}

#[tokio::test]
async fn test_v_gd() {
    let url = "https://v.gd/6H6dYQ";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("https://www.google.com/".to_string()));
}

#[tokio::test]
async fn test_virg_in() {
    let url = "https://virg.in/9sj";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(
        expanded_url,
        Ok("https://www.virginactive.co.za/quest".to_string())
    );
}

#[tokio::test]
async fn test_vzturl_com() {
    let url = "https://vzturl.com/bqd20";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.amazon.com/Sabrent-Thunderbolt-"));
}

#[tokio::test]
async fn test_waa_ai() {
    let url = "https://waa.ai/muZV";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.google.com/search?q=rust"));
}

#[tokio::test]
async fn test_yourwish_es() {
    let url = "http://yourwish.es/oxgyc";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert!(expanded_url
        .unwrap()
        .starts_with("https://www.amazon.com/Sabrent-Thunderbolt"));
}

#[tokio::test]
async fn test_zpr_io() {
    let url = "http://zpr.io/nniJB";
    let expanded_url = unshorten(url, None).await;
    assert!(expanded_url.is_ok());
    assert_eq!(expanded_url, Ok("http://www.archiveteam.org/".to_string()));
}
