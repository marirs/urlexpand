URLEXPAND
==========
![Crates.io](https://img.shields.io/crates/v/urlexpand)
[![Documentation](https://docs.rs/urlexpand/badge.svg)](https://docs.rs/urlexpand)
[![Build Status](https://travis-ci.com/marirs/urlexpand.svg?branch=main)](https://travis-ci.com/marirs/urlexpand)
[![GitHub license](https://img.shields.io/github/license/marirs/urlexpand)](https://github.com/marirs/urlexpand/blob/main/LICENSE)

Expand / Unshorten Shortened URL's.

### Example Usage

```toml
urlexpand = "0.0.3"
```

and then

```rust
use core::time::Duration;
use urlexpand::unshorten;

fn main() {
    let url = "https://bit.ly/3alqLKi";
    assert!(unshorten(url, Some(Duration::new(30,0))).is_some());   // with timeout
    assert!(unshorten(url, None).is_some());    // without timeout
}
```

### Running the example

```bash
cargo run --example unshorten
```

### Current list of URL Shortening services supported
- bit.ly
- buff.ly
- b.link
- chollo.to
- cutt.ly
- fa.by
- goo.gl - Google Service has now stopped their url shortening service 
- gns.io
- is.gd
- ldn.im
- rebrand.ly  
- rotf.lol
- snipr.com
- snipurl.com
- snurl.com
- t.co
- tiny.cc
- tinyurl.com
- tiny.one
- tr.im

### Contribution

Please feel free to contribute by making pull requests or even bug fixes.  
Thanks in advance.

---
License: MIT