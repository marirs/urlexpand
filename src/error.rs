#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("reqwest header error")]
    ReqwestHeader(String),
    #[error("reqwest error")]
    Reqwest(String),
    #[error("no string")]
    NoString,
    #[error("unknown error")]
    Unknown,
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(a: reqwest::header::ToStrError) -> Self {
        Self::ReqwestHeader(a.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(a: reqwest::Error) -> Self {
        Self::Reqwest(a.to_string())
    }
}
