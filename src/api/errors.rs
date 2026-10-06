use thiserror::Error;

#[derive(Debug, Error)]
#[error("printer error")]
pub enum ApiError {
    #[error("failed to decode response: {0}")]
    DecodingError(#[from] serde_json::Error),
    #[error("failed to perform request: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("failed to get priner info: {0}")]
    GetInfo(reqwest::Error),
}