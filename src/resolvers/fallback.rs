// FALLBACK Resolver - Hybrid approach using reqwest + curl fallback
use std::process::Command;
use std::time::Duration;

use crate::Result;
use crate::resolvers::generic;

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Expands URLs using a hybrid approach with curl fallback.
    //!
    //! This resolver first tries reqwest, and if that fails, falls back to using
    //! the curl command to get the redirect location. It's designed for services
    //! that work with curl but have issues with reqwest's HTTP client.
    //!
    //! # Arguments
    //!
    //! * `url` - The shortened URL to expand
    //! * `timeout` - Optional timeout for HTTP requests
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL on success,
    //! or `Err(Error)` if the URL cannot be expanded.
    //!
    //! # Behavior
    //!
    //! - First tries reqwest with custom user agent
    //! - Falls back to curl command if reqwest fails
    //! - Handles network errors and timeouts appropriately
    
    // Clone the URL to move it into the blocking task
    let url = url.to_string();
    
    // First try reqwest
    let reqwest_result = generic::unshort_with_curl_ua(&url, timeout).await;
    
    match reqwest_result {
        Ok(expanded_url) => {
            // Check if reqwest actually expanded the URL
            if expanded_url != url {
                Ok(expanded_url)
            } else {
                // reqwest didn't expand, try curl fallback
                curl_fallback(&url, timeout).await
            }
        }
        Err(_) => {
            // reqwest failed, try curl fallback
            curl_fallback(&url, timeout).await
        }
    }
}

async fn curl_fallback(url: &str, timeout: Option<Duration>) -> Result<String> {
    //! Fallback method using curl command to get redirect location.
    //!
    //! # Arguments
    //!
    //! * `url` - The URL to expand
    //! * `timeout` - Optional timeout for the curl command
    //!
    //! # Returns
    //!
    //! Returns `Ok(String)` with the final destination URL on success,
    //! or `Err(Error)` if the URL cannot be expanded.
    
    // Build curl command to get the final URL after redirects
    let mut cmd = Command::new("curl");
    cmd.arg("-s");
    cmd.arg("-L"); // Follow redirects
    cmd.arg("-w");
    cmd.arg("%{url_effective}"); // Show final URL
    cmd.arg("-o");
    cmd.arg("/dev/null"); // Discard body
    cmd.arg(url);
    
    // Set timeout if provided
    if let Some(timeout) = timeout {
        cmd.arg("--max-time");
        cmd.arg(timeout.as_secs().to_string());
    }
    
    // Execute curl command
    let output = cmd.output().map_err(|e| {
        // Check if curl command is not found
        if e.kind() == std::io::ErrorKind::NotFound {
            crate::Error::NoString
        } else {
            crate::Error::Reqwest(e.to_string())
        }
    })?;
    
    // Get the final URL from curl output
    let output_str = String::from_utf8_lossy(&output.stdout);
    let final_url = output_str.trim();
    
    if !final_url.is_empty() && final_url != url {
        Ok(final_url.to_string())
    } else {
        Err(crate::Error::NoString)
    }
}
