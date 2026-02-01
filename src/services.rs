//! Known URL shortener service registry.
//!
//! This module provides a static list of domains belonging to popular URL
//! shortening services and a helper function to detect whether a given URL
//! belongs to one of them.
//!
//! It is used as a **first-pass filter** before attempting expansion. By
//! identifying the shortening service early, the library can:
//!
//! - Route the URL to a service-specific resolver (when available)
//! - Apply generic redirect-following logic only when needed
//! - Avoid unnecessary work for already-expanded URLs
//!
//! ## How detection works
//!
//! The detection is string-based: if the URL contains any domain listed in
//! [`SERVICES`], it is considered to belong to a known shortener.
//!
//! This approach is fast and works well in practice, since shortener URLs
//! usually embed their domain directly in the visible URL.
//!
//! ## Limitations
//!
//! - Detection is **substring-based**, not a strict hostname match. This is
//!   intentional for performance and simplicity, but it may produce false
//!   positives in rare edge cases.
//! - Some shortening services use custom domains per customer. Those will not
//!   be detected unless added to this list.
//! - New shorteners appear frequently; this list may need periodic updates.
//!
//! ## Adding a new service
//!
//! To support a new shortener:
//!
//! 1. Add its domain to the [`SERVICES`] array
//! 2. (Optional) Implement a dedicated resolver module if it requires special
//!    handling beyond standard HTTP redirects
//!
//! ## Example
//!
//! ```ignore
//! let url = "https://bit.ly/abc123";
//! if let Some(service) = which_service(url) {
//!     println!("Shortened using: {service}");
//! }
//! ```
//!
//! If the URL does not match any known shortener domain, [`which_service`]
//! returns `None`.

/// List of domains for some known
/// URL shortening services.
pub(crate) static SERVICES: &[&str] = &[
    "adf.ly",
    "adfoc.us",
    "amzn.to",
    "amzn.id",
    "atominik.com",
    "ay.gy",
    "b.link",
    "bhpho.to",
    "bit.ly",
    "bit.do",
    "bn.gy",
    "branch.io",
    "buff.ly",
    "cal.link",
    "ceesty.com",
    "chatg.pt",
    "chollo.to",
    "cli.re",
    "cli.fm",
    "cutt.ly",
    "cutt.us",
    "db.tt",
    "dub.sh",
    "f.ls",
    "fa.by",
    "fb.me",
    "fig.page",
    "flip.it",
    "fumacrom.com",
    "geni.us",
    "ggl.link",
    "git.io",
    "git.new",
    "goo.gl",
    "goto.now",
    "gns.io",
    "hmm.rs",
    "ht.ly",
    "hyperurl.co",
    "is.gd",
    "intamema.com",
    "ity.im",
    "j.gs",
    "j.mp",
    "kutt.it",
    "2cm.es",
    "l1nq.com",
    "ldn.im",
    "linklyhq.com",
    "lnkd.in",
    "microify.com",
    "mzl.la",
    "nmc.sg",
    "nowlinks.net",
    "ow.ly",
    "plu.sh",
    "prf.hn",
    "q.gs",
    "qr.ae",
    "qr.net",
    "rb.gy",
    "rebrand.ly",
    "rlu.ru",
    "rotf.lol",
    "s.click.aliexpress.com",
    "s.coop",
    "s.id",
    "sh.st",
    "soo.gd",
    "short.gy",
    "shortcm.xyz",
    "shorturl.at",
    "sl1nk.com",
    "smu.sg",
    "smq.tc",
    "snip.ly",
    "snipr.com",
    "snipurl.com",
    "snurl.com",
    "split.to",
    "spti.fi",
    "surl.li",
    "t.co",
    "t.ly",
    "t2m.io",
    "tiny.cc",
    "tiny.pl",
    "tinyium.com",
    "tinyurl.com",
    "tinyurl.ae",
    "tiny.one",
    "tny.im",
    "tny.sh",
    "tr.im",
    "trib.al",
    "u.to",
    "v.gd",
    "virg.in",
    "vzturl.com",
    "waa.ai",
    "washex.am",
    "we.tl",
    "x.co",
    "y2u.be",
    "yt.vu",
    "yourwish.es",
    "zpr.io",
];

/// Check and tell which URL Shortner Service is used
pub(crate) fn which_service(url: &str) -> Option<&'static str> {
    SERVICES.iter().find(|&x| url.contains(x)).copied()
}
