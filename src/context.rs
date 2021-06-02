use crate::ElementHandler;
use crate::request::Request;
use crate::Url;
use crate::UrlManager;

#[derive(PartialEq)]
enum Status {
    INIT,
    RUNNING,
    FINISH,
}

pub struct SpiderContext {
    request: Request,
    url_manager: Box<dyn UrlManager>,
    element_handlers: Vec<Box<dyn ElementHandler>>,

    status: Status,
}

impl SpiderContext {
    pub fn new<U>(url_manager: U, element_handlers: Vec<Box<dyn ElementHandler>>) -> Self
        where U: UrlManager + 'static {
        Self {
            request: Request::new(),
            url_manager: Box::new(url_manager),
            element_handlers,
            status: Status::INIT,
        }
    }

    pub fn add_handler<H: ElementHandler + 'static>(&mut self, handler: H) {
        self.element_handlers.push(Box::new(handler));
    }

    pub fn push_url(&mut self, url: Url) -> bool {
        self.url_manager.push_url(url)
    }

    pub fn run(&mut self) {
        if self.status != Status::INIT {
            panic!("spider already running")
        }
        if self.element_handlers.len() == 0 {
            panic!("no handler")
        }

        while let Some(url) = self.url_manager.next_url() {
            let res = self.request.request_url(&url.url);
            if let Ok(ref ele) = res {
                for h in self.element_handlers.iter_mut() {
                    h.handle(self, ele);
                }
            }
        }
    }
}
