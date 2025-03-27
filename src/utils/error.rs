use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Environment variable missing: {0}")]
    EnvVarMissing(String), // Add a variant for missing env vars

    #[error("Internal server error")]
    InternalServerError,
}
