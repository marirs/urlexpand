/// List of domains for some known
/// URL shortening services.
pub(crate) static SERVICES: [&str; 23] = [
    "bit.ly",
    "buff.ly",
    "b.link",
    "chollo.to",
    "cutt.ly",
    "fa.by",
    "goo.gl",
    "gns.io",
    "is.gd",
    "iz4.short.gy",
    "ldn.im",
    "rebrand.ly",
    "rotf.lol",
    "snipr.com",
    "snipurl.com",
    "snurl.com",
    "split.to",
    "t.co",
    "tiny.cc",
    "tinyurl.com",
    "tiny.one",
    "tny.sh",
    "tr.im",
];

/// Check and tell which URL Shortner Service is used
pub(crate) fn which_service(url: &str) -> Option<&'static str> {
    SERVICES.iter().find(|&x| url.contains(x)).copied()
}
