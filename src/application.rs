use std::borrow::BorrowMut;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{RequestBuilder, UrlManager};
use crate::{Element, Url};
use crate::{ElementHandler, ErrorHandler, SpiderError};
use crate::request::Request;

#[derive(PartialEq)]
enum Status {
    INIT,
    RUNNING,
    FINISH,
}

pub struct SpiderApplication {
    request: Request,
    url_manager: Box<dyn UrlManager>,
    element_handlers: Vec<Box<dyn ElementHandler>>,
    error_handlers: Vec<Box<dyn ErrorHandler>>,

    status: Status,
}

impl SpiderApplication {
    pub fn new<U>(
        url_manager: U,
        element_handlers: Vec<Box<dyn ElementHandler>>,
        error_handlers: Vec<Box<dyn ErrorHandler>>,
        request_builder: Option<Box<dyn RequestBuilder>>,
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

        let max_task_num = 10;

        let (tx, mut rx) = mpsc::channel(10);
        let mut running = 0;

        running += self.try_boot_task(max_task_num, tx.clone()).await;

        while let Some(_) = rx.recv().await {
            running -= 1;

            running += self.try_boot_task(max_task_num - running, tx.clone()).await;

            if running == 0 {
                break;
            }
        }
    }

    // 尝试启动 num 个任务
    async fn try_boot_task(&mut self, num: i32, tx: mpsc::Sender<bool>) -> i32 {
        for i in 0..num {
            if let Some(url) = self.url_manager.next_url().await {
                tokio::spawn(async move {
                    match self.request.request_url(&url).await {
                        Ok(ref ele) => self.handle_element(&url, ele).await,
                        Err(ref e) => self.handle_err(&url, e).await,
                    }

                    tx.send(true).await
                });
            } else {
                return i;
            }
        }
        num
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
