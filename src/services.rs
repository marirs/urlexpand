/// List of domains for some known
/// URL shortening services.
pub(crate) static SERVICES: [&str; 24] = [
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
    "lurl.no",
    "moourl.com",
    "rebrand.ly",
    "rotf.lol",
    "smallr.com",
    "snipr.com",
    "snipurl.com",
    "snurl.com",
    "su.pr",
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
