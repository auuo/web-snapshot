use tokio::sync::mpsc;

use crate::Url;

pub struct SpiderContext {
    pub url_tx: mpsc::Sender<Url>,
}

impl SpiderContext {
    pub async fn push_url(&self, url: Url) {
        let _ = self.url_tx.send(url).await;
    }
}
