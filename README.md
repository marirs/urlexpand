URLEXPAND
==========
![Crates.io](https://img.shields.io/crates/v/urlexpand)
[![Documentation](https://docs.rs/urlexpand/badge.svg)](https://docs.rs/urlexpand)
[![Build Status](https://travis-ci.com/marirs/urlexpand.svg?branch=main)](https://travis-ci.com/marirs/urlexpand)
[![GitHub license](https://img.shields.io/github/license/marirs/urlexpand)](https://github.com/marirs/urlexpand/blob/main/LICENSE)

Expand / Unshorten an exhaustive list of Shortened URL's.

### Example Usage

- async
```toml
urlexpand = "0.2.9"
```

and then

```rust
use std::time::Duration;
use urlexpand::unshorten;

fn main() {
    let url = "https://bit.ly/3alqLKi";
    assert!(unshorten(url, Some(Duration::from_secs(10))).await.is_ok());   // with timeout
    assert!(unshorten(url, None).await.is_ok());    // without timeout
}
```

- blocking
```toml
urlexpand = { version = "0.2.9", features = ["blocking"] }
```

and then

```rust
use std::time::Duration;
use urlexpand::unshorten;

fn main() {
    let url = "https://bit.ly/3alqLKi";
    assert!(unshorten_blocking(url, Some(Duration::from_secs(10))).is_ok());   // with timeout
    assert!(unshorten_blocking(url, None).is_ok());    // without timeout
}
```

### Running the example

```bash
cargo run --example unshorten https://bit.ly/3alqLKi
```

### Current list of URL Shortening services supported
- `2cm.es` - 2CM / l8.nu
- `adf.ly` - Adfly
- `adfoc.us` - AdFocus  
- `amzn.to` - Amazon  
- `amzn.id` - Dub.sh
- `atominik.com` - part of Adf.ly 
- `ay.gy` -   part of Adf.ly
- `b.link` - BLINK
- `bhpho.to` - BH Photo & Video
- `bit.ly` - Bitly Url Shortner
- `bit.do` - Bitdo
- `bn.gy` - BNGY  
- `branch.io` - Branch.io  
- `buff.ly` - Buffer URL Shortner
- `cal.link` - Dub.sh
- `ceesty.com` - part of sh.st  
- `chatg.pt` - Dub.sh
- `chollo.to` - Chollo Ecommerce
- `cli.re` - Capsulink
- `cli.fm` - Capsulink  
- `cutt.ly` - Cuttly
- `cutt.us` - Cutt us  
- `db.tt` - Dropbox  
- `dub.sh` - Dub.sh
- `f.ls` - Free Link Shortener
- `fa.by` - part of rebrand.ly
- `fb.me` - Facebook  
- `fig.page` - Dub.sh
- `flip.it` - Flipboard  
- `fumacrom.com` - part of Adf.ly
- `ggl.link` - Dub.sh
- `git.io` - Github  
- `git.new` - Dub.sh
- `geni.us` - Genius Link
- `goo.gl` - Google Service has now stopped their url shortening service 
- `goto.now` - GOTO.NOW
- `gns.io` - part of trim (tr.im)
- `hmm.rs` - HMM.RS  
- `ht.ly` - part of Hootsuite  
- `hyperurl.co` - SmartUrl.It  
- `ity.im` - ity.im (it'-ee-i-am)
- `intamema.com` part of Adf.ly  
- `is.gd` - IS GD
- `j.gs` - part of Adf.ly
- `j.mp` - part of Bitly  
- `kutt.it` - Kutt
- `l1nq.com` - urlshort.dev
- `ldn.im` - part of trim (tr.im)
- `lnkd.in` - linkedin
- `linklyhq.com` - Linkly HQ
- `microify.com` - part of Adf.ly
- `mzl.la` - Mozilla Org  
- `nmc.sg` - instra corporation pty
- `nowlinks.net` - Now Links  
- `ow.ly` - part of Hootsuite  
- `prf.hn` -  Partnerize
- `plu.sh` - Plush  
- `q.gs` - part of Adfly  
- `qr.ae` - Quora  
- `qr.net` - QR Code URL shortner & generator  
- `rebrand.ly` - Rebrandly
- `rb.gy` - RBGY Free URL Shortner  
- `rlu.ru` - RLU.RU  
- `rotf.lol` - part of tinyurl.com
- `s.click.aliexpress.com` - Ali Express
- `s.coop` - SCOOP  
- `s.id` - SID (home.s.id)  
- `sh.st` - shorte.st
- `soo.gd` - Soo.Gd  
- `shortcm.xyz` - part of SHORT.IO  
- `short.gy` - SHORT.IO Service
- `shortcm.xyz` - ShortCm
- `shorturl.at` - ShortURL At  
- `smu.sg` - Singapore Management University  
- `smq.tc` - part of bit.ly  
- `snip.ly` - Sniply.io  
- `snipr.com`
- `snipurl.com`
- `snurl.com`
- `split.to` - Linksplit  
- `spti.fi` - Dub.sh
- `surl.li` - Hyperhost (Secom.com.ua)
- `t.co` - Twitter
- `t.ly` - T.LY Link Shortener  
- `t2m.io` - T2M aka "Text to Marketing" (t2mio.com)
- `tiny.cc` - TinyCC
- `tiny.pl` - TinyPL  
- `tinyium.com` - part of Adf.ly  
- `tinyurl.com` - TinyURL
- `tinyurl.ae` - TinyURL
- `tiny.one` - part of tinyurl.com
- `tny.im` - TNY.IM  
- `tny.sh` - Linksplit  
- `tr.im` - trim (tr.im) by RedLotus
- `trib.al` - Tribal links shortner  
- `u.to` - U TO  
- `v.gd` - V GD  
- `virg.in` - Virgin  
- `vzturl.com` - Vzt URL  
- `waa.ai` - Akari Link Shortner  
- `washex.am` - part of bit.ly  
- `x.co` - GoDaddy URL Shortner (currently shutdown)  
- `y2u.be` - YouTube URL Shortner by Firewrench inc.  
- `yt.vu` - YT.vu
- `yourwish.es` - Your Wishes  
- `zpr.io` - Zapier

### Contribution

Please feel free to contribute by making pull requests or even bug fixes.  
Thanks in advance.

---
License: MIT
