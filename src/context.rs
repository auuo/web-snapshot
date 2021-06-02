use crate::ElementHandler;
use crate::UrlManager;

#[derive(PartialEq)]
enum Status {
    INIT,
    RUNNING,
    FINISH,
}

pub struct SpiderContext {
    url_manager: Box<dyn UrlManager>,
    element_handlers: Vec<Box<dyn ElementHandler>>,

    status: Status,
}

impl SpiderContext {
    pub fn new<U>(url_manager: U, element_handlers: Vec<Box<dyn ElementHandler>>) -> Self
        where U: UrlManager + 'static {
        Self {
            url_manager: Box::new(url_manager),
            element_handlers,
            status: Status::INIT,
        }
    }

    pub fn add_handler<H: ElementHandler + 'static>(&mut self, handler: H) {
        self.element_handlers.push(Box::new(handler));
    }

    pub fn push_url(&mut self, url: String) -> bool {
        self.url_manager.push_url(url)
    }

    pub fn run(&mut self) {
        if self.status != Status::INIT {
            panic!("spider already running")
        }
        if self.element_handlers.len() == 0 {
            panic!("no handler")
        }
    }
}
