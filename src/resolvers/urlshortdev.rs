/// https://www.urlshort.dev Resolver
/// Expand urls that are shortened from urlshort.dev
use super::{from_re, get_client_builder};
use std::time::Duration;

use futures::future::{ready, TryFutureExt};
use serde::Deserialize;

use crate::{Error, Result};

static ENCURTADOR_CODE_RE: &str =
    r#"https?://(?:www\.)?encurtador\.dev/redirecionamento/([A-Za-z0-9]+)"#;

#[derive(Debug, Deserialize)]
struct DrApiResp {
    url: Option<String>,
}

pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    ready(get_client_builder(timeout).build())
        .map_err(Error::from)
        .and_then(|client| async move {
            let resp = client.get(url).send().await.map_err(Error::from)?;
            let final_url = resp.url().to_string();

            if let Some(code) = from_re(&final_url, ENCURTADOR_CODE_RE) {
                let api_url = format!("https://dr-api.encurtador.dev/encurtamentos/{}", code);

                let api_resp: DrApiResp = client
                    .get(api_url)
                    .header(reqwest::header::ACCEPT, "application/json,*/*")
                    .send()
                    .await
                    .map_err(Error::from)?
                    .error_for_status()
                    .map_err(Error::from)?
                    .json()
                    .await
                    .map_err(Error::from)?;

                return api_resp
                    .url
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().to_string())
                    .ok_or(Error::NoString);
            }

            Ok(final_url)
        })
        .await
}
