URLEXPAND
==========
![Crates.io](https://img.shields.io/crates/v/urlexpand)
[![Documentation](https://docs.rs/urlexpand/badge.svg)](https://docs.rs/urlexpand)
[![Build Status](https://travis-ci.com/marirs/urlexpand.svg?branch=main)](https://travis-ci.com/marirs/urlexpand)
[![GitHub license](https://img.shields.io/github/license/marirs/urlexpand)](https://github.com/marirs/urlexpand/blob/main/LICENSE)

Expand / Unshorten Shortened URL's.

### Example Usage

```toml
urlexpand = "0.0.6"
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
- bit.ly - Bitly Url Shortner
- bit.do  
- buff.ly - Buffer URL Shortner
- b.link - BLINK
- chollo.to
- cutt.ly
- fa.by - part of rebrand.ly
- git.io - Github  
- goo.gl - Google Service has now stopped their url shortening service 
- gns.io - part of trim (tr.im)
- is.gd
- iz4.short.gy - short.io Service  
- ldn.im - part of trim (tr.im)
- ow.ly - part of Hootsuite  
- rebrand.ly  
- rotf.lol - part of tinyurl.com
- sh.st - shorte.st
- snipr.com
- snipurl.com
- snurl.com
- split.to - Linksplit  
- t.co - Twitter
- tiny.cc
- tinyurl.com - TinyURL
- tiny.one - part of tinyurl.com
- tny.sh - Linksplit  
- tr.im - trim (tr.im)
- zpr.io - Zapier

### Contribution

Please feel free to contribute by making pull requests or even bug fixes.  
Thanks in advance.

---
License: MIT