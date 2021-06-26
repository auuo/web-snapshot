use crate::{Url, SpiderContext};

#[derive(thiserror::Error, Debug)]
pub enum SpiderError {
    #[error("http status error: {0}")]
    HttpStatus(reqwest::StatusCode),

    #[error("handle err: {0}")]
    HandleErr(anyhow::Error),

    #[error("unknown error: {0}")]
    Unknown(Box<dyn std::error::Error>),
}

impl From<reqwest::Error> for SpiderError {
    fn from(e: reqwest::Error) -> Self {
        Self::Unknown(Box::new(e))
    }
}

pub trait ErrorHandler {
    fn handle(&mut self, ctx: &mut SpiderContext, url: &Url, e: &SpiderError);
}