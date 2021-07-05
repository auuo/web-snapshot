use tokio::sync::mpsc;

use crate::application::Event;
use crate::Url;

pub struct SpiderContext {
    pub event_tx: mpsc::Sender<Event>,
}

impl SpiderContext {
    pub async fn push_url(&self, url: Url) {
        let _ = self.event_tx.send(Event::NewUrl(url)).await;
    }
}
