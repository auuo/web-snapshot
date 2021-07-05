use std::sync::Arc;

use tokio::sync::mpsc;

use crate::request::Request;
use crate::{Element, Url};
use crate::{ElementHandler, ErrorHandler, SpiderError};
use crate::{RequestBuilder, SpiderContext, UrlManager};

pub enum Event {
    NewUrl(Url),

    Finish,
}

pub struct SpiderApplication {
    request: Arc<Request>,
    url_manager: Box<dyn UrlManager>,
    element_handlers: Arc<Vec<Box<dyn ElementHandler>>>,
    error_handlers: Arc<Vec<Box<dyn ErrorHandler>>>,
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
            request: Arc::new(Request::new(request_builder)),
            url_manager: Box::new(url_manager),
            element_handlers: Arc::new(element_handlers),
            error_handlers: Arc::new(error_handlers),
        }
    }

    pub async fn push_url(&mut self, url: Url) -> bool {
        self.url_manager.push_url(url).await
    }

    pub async fn run(&mut self) {
        if self.element_handlers.len() == 0 {
            panic!("no handler")
        }

        let max_task_num = 10;
        let mut running = 0;

        let (event_tx, mut event_rx) = mpsc::channel(100);
        running += self.try_boot_task(max_task_num, event_tx.clone()).await;

        loop {
            match event_rx.recv().await {
                Some(Event::NewUrl(url)) => {
                    let _ = self.url_manager.push_url(url).await;
                }
                Some(Event::Finish) => {
                    running -= 1;

                    running += self
                        .try_boot_task(max_task_num - running, event_tx.clone())
                        .await;

                    // 事件按顺序发送，running 为 0 时后面不可能再有 url
                    if running == 0 {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    // 尝试启动 num 个任务
    async fn try_boot_task(&mut self, num: i32, event_tx: mpsc::Sender<Event>) -> i32 {
        for i in 0..num {
            if let Some(url) = self.url_manager.next_url().await {
                let event_tx = event_tx.clone();
                let element_handlers = self.element_handlers.clone();
                let error_handlers = self.error_handlers.clone();
                let request = self.request.clone();

                tokio::spawn(async move {
                    let mut ctx = SpiderContext { event_tx };
                    match request.request_url(&url).await {
                        Ok(ref ele) => {
                            SpiderApplication::handle_element(
                                &mut ctx,
                                &*element_handlers,
                                &*error_handlers,
                                &url,
                                ele,
                            )
                            .await
                        }
                        Err(ref e) => {
                            SpiderApplication::handle_err(&mut ctx, &*error_handlers, &url, e).await
                        }
                    }

                    let _ = ctx.event_tx.send(Event::Finish).await;
                });
            } else {
                return i;
            }
        }
        num
    }

    async fn handle_element(
        ctx: &mut SpiderContext,
        element_handlers: &Vec<Box<dyn ElementHandler>>,
        error_handlers: &Vec<Box<dyn ErrorHandler>>,
        url: &Url,
        ele: &Element,
    ) {
        for h in element_handlers.iter() {
            if let Err(e) = h.handle(ctx, url, ele).await {
                SpiderApplication::handle_err(ctx, error_handlers, url, &SpiderError::HandleErr(e))
                    .await;
            }
        }
    }

    async fn handle_err(
        ctx: &mut SpiderContext,
        error_handlers: &Vec<Box<dyn ErrorHandler>>,
        url: &Url,
        err: &SpiderError,
    ) {
        for h in error_handlers.iter() {
            h.handle(ctx, url, err).await;
        }
    }
}
