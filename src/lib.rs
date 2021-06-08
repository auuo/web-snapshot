#![feature(str_split_once)]

pub use context::SpiderContext;
pub use handler::Element;
pub use handler::ElementHandler;
pub use url_manager::BreadthFirstUrlManager;
pub use url_manager::Url;
pub use url_manager::UrlManager;
pub use error::SpiderError;

mod context;
mod handler;
mod url_manager;
mod request;
mod error;
