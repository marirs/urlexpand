/// List of domains for some known
/// URL shortening services.
pub(crate) static SERVICES: [&str; 31] = [
    "bit.ly",
    "bit.do",
    "buff.ly",
    "b.link",
    "chollo.to",
    "cutt.ly",
    "fa.by",
    "flip.it",
    "git.io",
    "goo.gl",
    "gns.io",
    "is.gd",
    "iz4.short.gy",
    "ldn.im",
    "nmc.sg",
    "ow.ly",
    "rebrand.ly",
    "rotf.lol",
    "sh.st",
    "smu.sg",
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
    "zpr.io",
];

/// Check and tell which URL Shortner Service is used
pub(crate) fn which_service(url: &str) -> Option<&'static str> {
    SERVICES.iter().find(|&x| url.contains(x)).copied()
}
