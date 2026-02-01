use thiserror::Error;

#[derive(Debug, Error)]
pub enum MoeqiError {
    #[error("invalid data: {0}")]
    InvalidData(&'static str),

    // --- compatibility with older code paths ---
    #[error("format error: {0}")]
    Format(&'static str),

    #[error("unsupported: {0}")]
    Unsupported(&'static str),

    #[error("unexpected EOF")]
    Eof,

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = core::result::Result<T, MoeqiError>;
