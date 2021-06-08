use bytes::Bytes;

use crate::{SpiderContext, Url};

#[derive(Debug)]
pub enum Element {
    HTML(String),
    JSON(String),
    TEXT { body: String, subtype: String },
    IMAGE { body: Bytes, subtype: String },
    OTHER { body: Bytes, c_type: String, subtype: String },
}

pub trait ElementHandler {
    fn handle(&mut self, ctx: &mut SpiderContext, url: &Url, ele: &Element);
}