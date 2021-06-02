use crate::Element;

pub struct Request {}

impl Request {
    pub fn new() -> Self {
        Self {}
    }

    pub fn request_url(&mut self, url: &String) -> anyhow::Result<Element> {
        Ok(Element::JSON("{\"name\": \"mac\"}".to_string()))
    }
}