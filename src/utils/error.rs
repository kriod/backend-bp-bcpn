use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Environment variable missing: {0}")]
    EnvVarMissing(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

impl ApiError {
    pub fn from_http_error(e: &reqwest::Error) -> Self {
        ApiError::HttpError(e.to_string())
    }
}
