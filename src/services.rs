/// List of domains for some known
/// URL shortening services.
pub(crate) static SERVICES: [&str; 23] = [
    "bit.ly",
    "buff.ly",
    "b.link",
    "chollo.to",
    "cli.gs",
    "cutt.ly",
    "fa.by",
    "is.gd",
    "lurl.no",
    "moourl.com",
    "smallr.com",
    "snipr.com",
    "snipurl.com",
    "snurl.com",
    "su.pr",
    "t.co",
    "tiny.cc",
    "tinyurl.com",
    "tiny.one",
    "rotf.lol",
    "tr.im",
    "gns.io",
    "ldn.im"
];

/// Check and tell which URL Shortner Service is used
pub(crate) fn which_service(url: &str) -> Option<&'static str> {
    SERVICES.iter().find(|&x| url.contains(x)).copied()
}
