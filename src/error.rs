#[derive(thiserror::Error, Debug)]
pub enum SpiderError {
    #[error("http status error: {0}")]
    HttpStatus(reqwest::StatusCode),

    #[error("unknown error: {0}")]
    Unknown(Box<dyn std::error::Error>),
}

impl From<reqwest::Error> for SpiderError {
    fn from(e: reqwest::Error) -> Self {
        Self::Unknown(Box::new(e))
    }
}
