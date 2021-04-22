/// List of domains for some known
/// URL shortening services.
pub(crate) static SERVICES: [&str; 20] = [
    "bit.ly",
    "buff.ly",
    "b.link",
    "chollo.to",
    "cutt.ly",
    "fa.by",
    "goo.gl",
    "gns.io",
    "is.gd",
    "ldn.im",
    "rebrand.ly",
    "rotf.lol",
    "snipr.com",
    "snipurl.com",
    "snurl.com",
    "t.co",
    "tiny.cc",
    "tinyurl.com",
    "tiny.one",
    "tr.im",
];

/// Check and tell which URL Shortner Service is used
pub(crate) fn which_service(url: &str) -> Option<&'static str> {
    SERVICES.iter().find(|&x| url.contains(x)).copied()
}
