URLEXPAND
==========
![Crates.io](https://img.shields.io/crates/v/urlexpand)
[![Documentation](https://docs.rs/urlexpand/badge.svg)](https://docs.rs/urlexpand)
[![Build Status](https://travis-ci.com/marirs/urlexpand.svg?branch=main)](https://travis-ci.com/marirs/urlexpand)
[![GitHub license](https://img.shields.io/github/license/marirs/urlexpand)](https://github.com/marirs/urlexpand/blob/main/LICENSE)

Expand / Unshorten Shortened URL's.

### Example Usage

```toml
urlexpand = "0.0.8"
```

and then

```rust
use std::time::Duration;
use urlexpand::unshorten;

fn main() {
    let url = "https://bit.ly/3alqLKi";
    assert!(unshorten(url, Some(Duration::from_secs(10))).is_some());   // with timeout
    assert!(unshorten(url, None).is_some());    // without timeout
}
```

### Running the example

```bash
cargo run --example unshorten
```

### Current list of URL Shortening services supported
- `adf.ly` - Adfly
- `atominik.com` - part of Adf.ly 
- `b.link` - BLINK
- `bit.ly` - Bitly Url Shortner
- `bit.do` - Bitdo
- `bn.gy` - BNGY  
- `buff.ly` - Buffer URL Shortner
- `chollo.to` - Chollo Ecommerce
- `cutt.ly` - Cuttly
- `fa.by` - part of rebrand.ly
- `fb.me` - Facebook  
- `flip.it` - Flipboard  
- `fumacrom.com` - part of Adf.ly  
- `git.io` - Github  
- `goo.gl` - Google Service has now stopped their url shortening service 
- `gns.io` - part of trim (tr.im)
- `hmm.rs` - HMM.RS  
- `intamema.com` part of Adf.ly  
- `is.gd` - IS GD
- `iz4.short.gy` - short.io Service  
- `j.gs` - part of Adf.ly
- `kutt.it` - Kutt
- `ldn.im` - part of trim (tr.im)
- `nmc.sg` - instra corporation pty
- `nowlinks.net` - Now Links  
- `ow.ly` - part of Hootsuite  
- `q.gs` - part of Adfly  
- `rebrand.ly` - Rebrandly
- `rlu.ru` - RLU.RU  
- `rotf.lol` - part of tinyurl.com
- `s.coop` - SCOOP  
- `sh.st` - shorte.st
- `smu.sg` - Singapore Management University  
- `snipr.com`
- `snipurl.com`
- `snurl.com`
- `split.to` - Linksplit  
- `t.co` - Twitter
- `tiny.cc` - TinyCC
- `tinyurl.com` - TinyURL
- `tiny.one` - part of tinyurl.com
= `tny.im` - TNY.IM  
- `tny.sh` - Linksplit  
- `tr.im` - trim (tr.im)
- `v.gd` - V GD  
- `zpr.io` - Zapier

### Contribution

Please feel free to contribute by making pull requests or even bug fixes.  
Thanks in advance.

---
License: MIT