use crate::request::Request;
use crate::UrlManager;
use crate::{Element, Url};
use crate::{ElementHandler, ErrorHandler, SpiderError};

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
        request_builder: Option<Box<dyn Fn(&Url) -> reqwest::Result<reqwest::Response> + Send>>,
    ) -> Self
    where
        U: UrlManager + 'static,
    {
        Self {
            request: Request::new(request_builder),
            url_manager: Box::new(url_manager),
            element_handlers,
            error_handlers,
            status: Status::INIT,
        }
    }

    pub fn add_handler<H: ElementHandler + 'static>(&mut self, handler: H) {
        self.element_handlers.push(Box::new(handler));
    }

    pub async fn push_url(&mut self, url: Url) -> bool {
        self.url_manager.push_url(url).await
    }

    pub async fn run(&mut self) {
        if self.status != Status::INIT {
            panic!("spider already running")
        }
        if self.element_handlers.len() == 0 {
            panic!("no handler")
        }

        while let Some(url) = self.url_manager.next_url().await {
            match self.request.request_url(&url).await {
                Ok(ref ele) => self.handle_element(&url, ele).await,
                Err(ref e) => self.handle_err(&url, e).await,
            }
        }
    }

    async fn handle_element(&mut self, url: &Url, ele: &Element) {
        unsafe {
            let s = self as *mut Self;
            for h in (*s).element_handlers.iter_mut() {
                if let Err(e) = h.handle(self, url, ele).await {
                    self.handle_err(url, &SpiderError::HandleErr(e)).await;
                }
            }
        }
    }

    async fn handle_err(&mut self, url: &Url, err: &SpiderError) {
        unsafe {
            let s = self as *mut Self;
            for h in (*s).error_handlers.iter_mut() {
                h.handle(self, url, err).await;
            }
        }
    }
}
