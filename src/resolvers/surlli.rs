// SURL.LI Resolver
use crate::resolvers::{from_url, generic};
use crate::{Error, Result};
use std::time::Duration;

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs shortened by surl.li.
    //!
    //! This resolver handles surl.li's two-stage redirect process:
    //! 1. First attempts standard HTTP redirect following
    //! 2. If the URL doesn't change, parses HTML for API-based redirect
    //!
    //! # Arguments
    //!
    //! * `url` - The surl.li shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL on success,
    //! or `Err(Error)` if the URL cannot be expanded.
    //!
    //! # Behavior
    //!
    //! - First tries standard HTTP redirect following
    //! - If no expansion occurs, parses HTML for API redirect URL
    //! - Extracts the final URL from api.miniature.io calls
    let expanded_url = generic::unshort(url, timeout).await?;
    Ok(
        if expanded_url.ends_with(url) {
            // No redirect occurred (generic resolver just added scheme), need to parse HTML for the real URL
            match get_from_html(url, timeout).await {
                Ok(u) => u,
                Err(_) => expanded_url,
            }
        } else {
            // Proper redirect occurred, use the expanded URL
            expanded_url
        },
    )
}

async fn get_from_html(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Extracts the final URL from surl.li's HTML page.
    //!
    //! This function parses the HTML content to find the direct link
    //! that contains the actual destination URL.
    //!
    //! # Arguments
    //!
    //! * `url` - The surl.li URL to parse
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the extracted destination URL, or
    //! `Err(Error)` if the URL cannot be extracted.
    //!
    //! # Behavior
    //!
    //! - Fetches the HTML content of the page
    //! - Searches for direct link in the HTML
    //! - Extracts the final URL from the href attribute
    let html = from_url(url, timeout).await?;
    
    // Look for the "To direct link" pattern
    if let Some(start) = html.find("To direct link") {
        // Look backwards to find the href attribute
        let before_link = &html[..start];
        if let Some(href_start) = before_link.rfind("href=\"") {
            let href_content = &before_link[href_start + 6..];
            if let Some(href_end) = href_content.find("\"") {
                let extracted_url = &href_content[..href_end];
                if !extracted_url.is_empty() && (extracted_url.starts_with("http://") || extracted_url.starts_with("https://")) {
                    return Ok(extracted_url.to_string());
                }
            }
        }
    }
    
    // Fallback to other patterns
    let patterns = [
        "api.miniature.io/?url=",
        "api.miniature.io?url=",
        "\"url\":\"",
        "url=",
    ];
    
    for pattern in &patterns {
        if let Some(result) = html.split(pattern).last().and_then(|r| r.split('"').next()) {
            let extracted_url = result.to_string();
            if !extracted_url.is_empty() && (extracted_url.starts_with("http://") || extracted_url.starts_with("https://")) {
                return Ok(extracted_url);
            }
        }
    }
    
    Err(Error::NoString)
}
