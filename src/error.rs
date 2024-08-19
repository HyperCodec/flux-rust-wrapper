use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Image decoding failed: {0}")]
    Image(#[from] image::ImageError),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;