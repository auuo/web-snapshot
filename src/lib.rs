#![feature(str_split_once)]

use thiserror::Error;

pub use context::SpiderContext;
pub use handler::Element;
pub use handler::ElementHandler;
pub use url_manager::BreadthFirstUrlManager;
pub use url_manager::Url;
pub use url_manager::UrlManager;

mod context;
mod handler;
mod url_manager;
mod request;

#[derive(Error, Debug)]
pub enum SpiderError {
    #[error("http status error: {0}")]
    HttpStatus(reqwest::StatusCode),

    #[error("unknown error: {0}")]
    Unknown(Box<dyn std::error::Error>),
}