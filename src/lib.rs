use core::time::Duration;

mod resolvers;

mod services;
use services::{SERVICES, which_service};

pub fn is_shortened(url: &str) -> bool {
    //! Check to see if a given url is a shortened url
    //! ## Example
    //! ```rust
    //! use urlexpand::is_shortened;
    //!
    //! fn main() {
    //!     let url = "https://bit.ly/id";
    //!     assert!(is_shortened(url));
    //! }
    //! ```
    SERVICES.iter().any(|x| url.contains(x))
}

pub fn unshorten(url: &str, timeout: Option<Duration>) -> Option<String> {
    //! UnShorten a shortened URL
    //! ## Example
    //! ```rust
    //! use core::time::Duration;
    //! use urlexpand::unshorten;
    //!
    //! fn main() {
    //!     let url = "https://bit.ly/3alqLKi";
    //!     assert!(unshorten(url, Some(Duration::new(30,0))).is_some());   // with timeout
    //!     assert!(unshorten(url, None).is_some());    // without timeout
    //! }
    //! ```
    let service = match which_service(url) {
        Some(service) => service,
        None => return None,
    };

    match service {
        "tinyurl.com" => resolvers::tinyurl::unshort(url, timeout),
        "t.co" => resolvers::custom::unshort(url, timeout),
        _ => resolvers::generic::unshort(url, timeout),
    }
}

#[cfg(test)]
mod tests {
    use super::unshorten;

    #[test]
    fn test_bit_ly() {
        let url = "https://bit.ly/3alqLKi";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
    }

    #[test]
    fn test_b_link() {
        let url = "https://b.link/cx2x2l";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert_eq!(expanded_url, Some("http://www.google.com/".to_string()));
    }

    #[test]
    fn test_cutt_ly() {
        let url = "https://cutt.ly/tvDqE79";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
    }

    #[test]
    fn test_tiny_cc() {
        let url = "http://tiny.cc/5ocwtz";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
    }

    #[test]
    fn test_tinurl_com() {
        let url = "https://tinyurl.com/2j582c6a";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert_eq!(expanded_url, Some("https://google.com".to_string()));
    }

    #[test]
    fn test_t_co() {
        let url = "https://t.co/bYeHhy9kAU";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert_eq!(expanded_url, Some("https://www.youtube.com/watch?v=x6QZn9xiuOE".to_string()));
    }

    #[test]
    fn tiny_one() {
        let url = "https://tiny.one/f94uhh4x";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
    }

    #[test]
    fn test_rotf_lol() {
        let url = "https://rotf.lol/4scu3nzz";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert_eq!(expanded_url, Some("https://www.google.com/".to_string()));
    }

    #[test]
    fn test_tr_im() {
        let url = "https://tr.im/1iMz2";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert!(expanded_url.unwrap().starts_with("https://www.google.com/"));
    }

    #[test]
    fn test_gns_io() {
        let url = "https://gns.io/1RQl2";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert!(expanded_url.unwrap().starts_with("https://www.google.com/"));
    }

    #[test]
    fn test_ldn_im() {
        let url = "http://ldn.im/1pNey";
        let expanded_url = unshorten(url, None);
        assert!(expanded_url.is_some());
        assert!(expanded_url.unwrap().starts_with("https://www.google.com/"));
    }
}
