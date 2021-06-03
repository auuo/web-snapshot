use crate::{SpiderContext, Url};

#[derive(Debug)]
pub enum Element {
    HTML(String),
    PLAIN(String),
    JSON(String),
    IMAGE(Vec<u8>),
    OTHER(Vec<u8>),
}

pub trait ElementHandler {
    fn handle(&mut self, ctx: &mut SpiderContext, url: &Url, ele: &Element);
}