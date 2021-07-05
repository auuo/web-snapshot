use async_trait::async_trait;

use crate::{SpiderApplication, Url};

#[derive(thiserror::Error, Debug)]
pub enum SpiderError {
    #[error("http status error: {0}")]
    HttpStatus(reqwest::StatusCode),

    #[error("handle err: {0}")]
    HandleErr(anyhow::Error),

    #[error("unknown error: {0}")]
    Unknown(anyhow::Error),
}

impl From<reqwest::Error> for SpiderError {
    fn from(e: reqwest::Error) -> Self {
        Self::Unknown(anyhow::Error::from(e))
    }
}

#[async_trait]
pub trait ErrorHandler: Send + Sync {
    async fn handle(&self, ctx: &mut SpiderApplication, url: &Url, e: &SpiderError);
}
