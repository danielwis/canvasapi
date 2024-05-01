use thiserror::Error;

#[derive(Error, Debug)]
pub enum CanvasError {
    #[error("JSON parse error: {0}")]
    Client(#[from] serde_json::Error),
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}

