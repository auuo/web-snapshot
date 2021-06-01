pub enum Element {
    HTML(String),
    PLAIN(String),
    JSON(String),
    IMAGE(Vec<u8>),
    OTHER(Vec<u8>),
}

pub trait UrlManager {
    fn push_url(&mut self, url: String) -> bool;

    fn next_url(&mut self) -> Option<String>;
}

pub trait ElementHandler {
    fn handle(&mut self, ele: Element);
}

pub struct SpiderContext {
    url_manager: Box<dyn UrlManager>,
    element_handlers: Vec<Box<dyn ElementHandler>>,
}

impl SpiderContext {
    fn new<U>(url_manager: U, element_handlers: Vec<Box<dyn ElementHandler>>) -> Self
        where U: UrlManager + 'static {
        Self {
            url_manager: Box::new(url_manager),
            element_handlers,
        }
    }

    fn add_handler<H: ElementHandler + 'static>(&mut self, handler: H) {
        self.element_handlers.push(Box::new(handler));
    }

    fn push_url(&mut self, url: String) -> bool {
        self.url_manager.push_url(url)
    }
}

fn main() {}