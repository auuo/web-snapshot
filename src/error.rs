use crate::{Url, SpiderContext};

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

pub trait ErrorHandler {
    fn handle(&self, ctx: &mut SpiderContext, url: &Url, e: &SpiderError);
}