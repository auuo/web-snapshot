use crate::{ElementHandler, ErrorHandler};
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
    error_handlers: Vec<Box<dyn ErrorHandler>>,

    status: Status,
}

impl SpiderContext {
    pub fn new<U>(
        url_manager: U,
        element_handlers: Vec<Box<dyn ElementHandler>>,
        error_handlers: Vec<Box<dyn ErrorHandler>>,
    ) -> Self
        where U: UrlManager + 'static {
        Self {
            request: Request::new(),
            url_manager: Box::new(url_manager),
            element_handlers,
            error_handlers,
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

        unsafe {
            let s = self as *mut Self;
            while let Some(url) = (*s).url_manager.next_url() {
                match (*s).request.request_url(&url.url) {
                    Ok(ref ele) => {
                        for h in (*s).element_handlers.iter_mut() {
                            h.handle(&mut *s, url, ele);
                        }
                    }
                    Err(ref e) => {
                        for h in (*s).error_handlers.iter_mut() {
                            h.handle(&mut *s, url, e);
                        }
                    }
                }
            }
        }
    }
}
