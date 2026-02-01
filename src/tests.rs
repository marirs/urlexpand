use super::{is_shortened, unshorten, validate};
#[cfg(feature = "blocking")]
use super::unshorten_async;
use std::time::Duration;

/// Test data for URL expansion
struct TestCase {
    name: &'static str,
    short_url: &'static str,
    expected_contains: &'static str,
    exact_match: bool,
}

impl TestCase {
    const fn new(name: &'static str, short_url: &'static str, expected: &'static str) -> Self {
        Self {
            name,
            short_url,
            expected_contains: expected,
            exact_match: true,
        }
    }

    const fn starts_with(name: &'static str, short_url: &'static str, expected: &'static str) -> Self {
        Self {
            name,
            short_url,
            expected_contains: expected,
            exact_match: false,
        }
    }
}

const TEST_CASES: &[TestCase] = &[
    TestCase::new("bit_ly", "https://bit.ly/3alqLKi", "https://www.google.com/"),
    TestCase::new("b_link", "https://b.link/cx2x2l", "http://www.google.com/"),
    TestCase::new("cutt_ly", "https://cutt.ly/tvDqE79", "https://www.google.com/"),
    TestCase::new("git_io", "https://git.io/JOiM6", "https://github.com/marirs/urlexpand"),
    TestCase::starts_with("googl", "https://goo.gl/cvSjeY", "http://m.axisbank.com"),
    TestCase::starts_with("2cm_es", "https://2cm.es/1", "https://antiphishing.biz/2021/11/10/fake-microsoft-windows-11-detected/"),
    TestCase::new("kutt_it", "https://kutt.it/jO2XmP", "https://www.google.com/"),
    TestCase::new("rb_gy", "https://rb.gy/ciq6si", "https://www.google.com/"),
    TestCase::starts_with("surl_li", "surl.li/aap", "http://google.com"),
    TestCase::new("t_co", "https://t.co/bYeHhy9kAU", "https://www.youtube.com/watch?v=x6QZn9xiuOE"),
    TestCase::new("tny_sh", "https://tny.sh/5C3X9Ss", "http://www.google.com/"),
    TestCase::starts_with("ow_ly", "https://ow.ly/3alqLKi", "http://atacmobile.it/atacmobile.php?service=news&action=single&newsid=2128"),
    TestCase::new("fb_me", "https://fb.me/3alqLKi", "https://www.facebook.com/"),
];

/// Helper function to test URL expansion
fn test_expansion(result: super::Result<String>, test_case: &TestCase) -> std::result::Result<(), String> {
    match result {
        Ok(expanded_url) => {
            let success = if test_case.exact_match {
                expanded_url == test_case.expected_contains
            } else {
                expanded_url.starts_with(test_case.expected_contains)
            };

            if success {
                Ok(())
            } else {
                Err(format!(
                    "{}: Expected {} '{}', got '{}'",
                    test_case.name,
                    if test_case.exact_match { "exact match" } else { "starts with" },
                    test_case.expected_contains,
                    expanded_url
                ))
            }
        }
        Err(e) => {
            Err(format!("{}: Request failed: {}", test_case.name, e))
        }
    }
}

// Basic unit tests
#[test]
fn test_validate() {
    assert!(validate("bit.ly").is_some());
    assert!(validate("https://bit.ly/").is_some());
    assert!(validate("bit").is_none());
    assert!(validate("https://bit").is_none());
    assert!(validate("google.com").is_none());
    assert!(validate("google").is_none());
}

#[test]
fn test_is_shortened() {
    let url = "https://bit.ly/3alqLKi";
    assert!(is_shortened(url));
    let url = "https://www.google.com";
    assert!(!is_shortened(url));
}

// Async tests (always available)
#[cfg(not(feature = "blocking"))]
mod async_tests {
    use super::*;

    #[tokio::test]
    async fn test_async_expansions() {
        for test_case in TEST_CASES {
            let result = unshorten(test_case.short_url, Some(Duration::from_secs(10))).await;
            if let Err(msg) = test_expansion(result, test_case) {
                panic!("{}", msg);
            }
        }
    }

    #[tokio::test]
    async fn test_async_with_timeout() {
        let result = unshorten("https://bit.ly/3alqLKi", Some(Duration::from_secs(5))).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_no_timeout() {
        let result = unshorten("https://bit.ly/3alqLKi", None).await;
        assert!(result.is_ok());
    }
}

// Tests when blocking feature is enabled
#[cfg(feature = "blocking")]
mod blocking_tests {
    use super::*;

    #[test]
    fn test_blocking_expansions() {
        for test_case in TEST_CASES {
            let result = unshorten(test_case.short_url, Some(Duration::from_secs(10)));
            if let Err(msg) = test_expansion(result, test_case) {
                panic!("{}", msg);
            }
        }
    }

    #[test]
    fn test_blocking_with_timeout() {
        let result = unshorten("https://bit.ly/3alqLKi", Some(Duration::from_secs(5)));
        assert!(result.is_ok());
    }

    #[test]
    fn test_blocking_no_timeout() {
        let result = unshorten("https://bit.ly/3alqLKi", None);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_still_available() {
        // Test that async version is still available when blocking feature is enabled
        let result = unshorten_async("https://bit.ly/3alqLKi", Some(Duration::from_secs(5))).await;
        assert!(result.is_ok());
    }
}

// Integration tests that work for both configurations
#[cfg(feature = "blocking")]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_async_available_with_blocking_feature() {
        // Test that async version is still available when blocking feature is enabled
        let result = unshorten_async("https://bit.ly/3alqLKi", Some(Duration::from_secs(5))).await;
        assert!(result.is_ok());
    }
}

// Error handling tests
mod error_tests {
    use super::*;

    #[cfg(not(feature = "blocking"))]
    #[tokio::test]
    async fn test_async_invalid_url() {
        let result = unshorten("https://example.com/not-a-shortener", Some(Duration::from_secs(5))).await;
        assert!(result.is_err());
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn test_blocking_invalid_url() {
        let result = unshorten("https://example.com/not-a-shortener", Some(Duration::from_secs(5)));
        assert!(result.is_err());
    }

    #[cfg(feature = "blocking")]
    #[tokio::test]
    async fn test_async_invalid_url_with_blocking_feature() {
        let result = unshorten_async("https://example.com/not-a-shortener", Some(Duration::from_secs(5))).await;
        assert!(result.is_err());
    }
}
