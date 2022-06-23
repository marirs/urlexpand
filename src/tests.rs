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
    ($t_name:ident, $s_url:expr, $op:ident, $e_url:expr) => {
        #[tokio::test]
        async fn $t_name() {
            let expanded_url = unshorten($s_url, None).await;
            assert_eq!(
                expanded_url.as_ref().map(|x| x.$op($e_url)),
                Ok(true),
                "{}: {:?} {} {}",
                $s_url,
                expanded_url,
                stringify!($op),
                $e_url
            );
        }

        // until std::concat_idents stablizes
        paste! {
            #[test]
            fn [<$t_name _blocking>]() {
                let expanded_url = unshorten_blocking($s_url, None);
                assert_eq!(
                    expanded_url.as_ref().map(|x| x.$op($e_url)),
                    Ok(true),
                    "{}: {:?} {} {}",
                    $s_url,
                    expanded_url,
                    stringify!($op),
                    $e_url
                );
            }
        }
    };
}

test_shorten_link!(
    test_bit_ly,
    "https://bit.ly/3alqLKi",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_bit_do,
    "http://bit.do/fQy4h",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_b_link,
    "https://b.link/cx2x2l",
    eq,
    "http://www.google.com/"
);

test_shorten_link!(
    test_buff_ly,
    "https://buff.ly/1GYcFvQ",
    starts_with,
    "https://blog.bufferapp.com/url-shorteners?"
);

test_shorten_link!(
    test_cli_re,
    "https://cli.re/wxbz38",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_chollo_to,
    "https://chollo.to/s1q4u",
    starts_with,
    "https://www.chollometro.com/ofertas/xiaomi-mi-band-6"
);

test_shorten_link!(
    test_cutt_ly,
    "https://cutt.ly/tvDqE79",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_db_tt,
    "https://db.tt/cchelp",
    starts_with,
    "https://www.dropbox.com/"
);

test_shorten_link!(
    test_fb_me,
    "https://fb.me/mashable",
    eq,
    "https://www.facebook.com/mashable"
);

test_shorten_link!(
    test_git_io,
    "https://git.io/JOiM6",
    eq,
    "https://www.github.com/marirs/urlexpand"
);

test_shorten_link!(
    test_goo_gl,
    "https://goo.gl/cvSjeY",
    starts_with,
    "http://m.axisbank.com/"
);

test_shorten_link!(
    test_gns_io,
    "https://gns.io/1RQl2",
    starts_with,
    "https://google.com/"
);

test_shorten_link!(
    test_hmm_rs,
    "http://hmm.rs/Hangfire.PostgreSql",
    starts_with,
    "https://github.com/"
);

test_shorten_link!(
    test_hyperurl_co,
    "http://hyperurl.co/qicb73",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_ity_im,
    "http://ity.im/U8re4",
    starts_with,
    "https://www.google.com/search?q=rust&"
);

test_shorten_link!(
    test_iz4_short_gy,
    "https://iz4.short.gy/mr7KcJ",
    starts_with,
    "https://www.google.com"
);

test_shorten_link!(
    test_kutt_it,
    "https://kutt.it/jO2XmP",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_ldn_im,
    "http://ldn.im/1pNey",
    starts_with,
    "https://google.com/"
);

test_shorten_link!(
    test_linklyhq_com,
    "https://l.linklyhq.com/l/QebZ",
    starts_with,
    "https://www.google.com/"
);

test_shorten_link!(
    test_mlz_la,
    "https://mzl.la/3eqJ565",
    eq,
    "http://support.mozilla.org/"
);

test_shorten_link!(
    test_ow_ly,
    "http://ow.ly/j9qh7",
    eq,
    "http://t.co/cAcQV4QTOS"
);

test_shorten_link!(
    test_plu_sh,
    "https://plu.sh/xnwb8",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_qr_ae,
    "http://qr.ae/7FQS9",
    starts_with,
    "http://www.quora.com"
);

test_shorten_link!(
    test_rb_gy,
    "https://rb.gy/ciq6si",
    eq,
    "https://google.com/"
);

test_shorten_link!(
    test_rebrand_ly,
    "https://rebrand.ly/dp8cuo0",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_rlu_ru,
    "https://rlu.ru/1B5",
    eq,
    "https://www.google.com"
);

test_shorten_link!(
    test_rotf_lol,
    "https://rotf.lol/4scu3nzz",
    eq,
    "https://google.com/"
);

test_shorten_link!(
    test_shorturl_at,
    "https://shorturl.at/kmrEO",
    eq,
    "https://www.google.com"
);

test_shorten_link!(
    test_split_to,
    "https://split.to/V1ZhFut",
    starts_with,
    "https://google.com"
);

test_shorten_link!(
    test_smc_sg,
    "https://smu.sg/4l4",
    starts_with,
    "https://google.com/"
);

test_shorten_link!(
    test_snip_ly,
    "snip.ly/soyummy-cookbook",
    starts_with,
    "https://www.soyummystore.com"
);

test_shorten_link!(
    test_surl_ly_redirect,
    "surl.li/cgonw",
    starts_with,
    "https://bing.com/"
);

test_shorten_link!(
    test_surl_ly,
    "surl.li/aap",
    starts_with,
    "http://google.com"
);

test_shorten_link!(
    test_t_co,
    "https://t.co/bYeHhy9kAU",
    eq,
    "https://www.youtube.com/watch?v=x6QZn9xiuOE"
);

test_shorten_link!(
    test_t_ly,
    "https://t.ly/2ESW",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_t2m_io,
    "https://t2m.io/SSQhKqJ2",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_tiny_cc,
    "https://tiny.cc/5ocwtz",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_tinurl_com,
    "https://tinyurl.com/2j582c6a",
    eq,
    "https://google.com"
);

test_shorten_link!(
    test_tiny_one,
    "https://tiny.one/f94uhh4x",
    eq,
    "https://google.com/"
);

test_shorten_link!(
    test_tny_sh,
    "https://tny.sh/5C3X9Ss",
    eq,
    "http://google.com/"
);

test_shorten_link!(
    test_tr_im,
    "https://tr.im/1iMz2",
    starts_with,
    "https://google.com/"
);

test_shorten_link!(
    test_trib_al,
    "https://trib.al/YKNecc2",
    starts_with,
    "https://mashable.com/article"
);

test_shorten_link!(
    test_u_to,
    "https://u.to/P05FGw",
    starts_with,
    "https://www.google.com/search?q=rust"
);

test_shorten_link!(
    test_v_gd,
    "https://v.gd/6H6dYQ",
    eq,
    "https://www.google.com/"
);

test_shorten_link!(
    test_virg_in,
    "https://virg.in/9sj",
    eq,
    "https://www.virginactive.co.za/quest"
);

test_shorten_link!(
    test_vzturl_com,
    "https://vzturl.com/bqd20",
    starts_with,
    "https://www.amazon.com/Sabrent-Thunderbolt-"
);

test_shorten_link!(
    test_waa_ai,
    "https://waa.ai/muZV",
    starts_with,
    "https://www.google.com/search?q=rust"
);

test_shorten_link!(
    test_yourwish_es,
    "http://yourwish.es/oxgyc",
    starts_with,
    "https://www.amazon.com/Sabrent-Thunderbolt"
);

test_shorten_link!(
    test_zpr_io,
    "http://zpr.io/nniJB",
    eq,
    "http://www.archiveteam.org/"
);
