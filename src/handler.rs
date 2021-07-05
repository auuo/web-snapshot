use async_trait::async_trait;
use bytes::Bytes;

use crate::{SpiderApplication, Url};

#[derive(Debug)]
pub enum Element {
    HTML(String),
    JSON(String),
    TEXT { body: String, subtype: String },
    IMAGE { body: Bytes, subtype: String },
    OTHER { body: Bytes, c_type: String, subtype: String },
}

#[async_trait]
pub trait ElementHandler: Send + Sync {
    async fn handle(&self, ctx: &mut SpiderApplication, url: &Url, ele: &Element) -> anyhow::Result<()>;
}
