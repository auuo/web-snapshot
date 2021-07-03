use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;

use crate::{Element, SpiderError, Url};

pub struct Request {
    request_builder: Option<Box<dyn RequestBuilder>>,
}

#[async_trait]
pub trait RequestBuilder: Send + Sync {
    async fn build(&self, url: &Url) -> reqwest::Result<reqwest::Response>;
}

impl Request {
    pub fn new(request_builder: Option<Box<dyn RequestBuilder>>) -> Self {
        Self { request_builder }
    }

    pub async fn request_url(&self, url: &Url) -> Result<Element, SpiderError> {
        let resp = if let Some(ref rb) = self.request_builder {
            rb.build(url).await?
        } else {
            reqwest::get(&url.url).await?
        };

        if !resp.status().is_success() {
            return Err(SpiderError::HttpStatus(resp.status()));
        }

        let option = resp
            .headers()
            .get(CONTENT_TYPE)
            .map(|h| h.to_str().unwrap_or(""));

        let ele = match option {
            Some(t) if t.starts_with("application/json") => Element::JSON(resp.text().await?),

            Some(t) if t.starts_with("text/html") => Element::HTML(resp.text().await?),

            Some(t) if t.starts_with("text/") => Element::TEXT {
                subtype: t.strip_prefix("text/").unwrap().to_string(),
                body: resp.text().await?,
            },

            Some(t) if t.starts_with("image/") => Element::IMAGE {
                subtype: t.strip_prefix("image/").unwrap().to_string(),
                body: resp.bytes().await?,
            },

            Some(t) => Element::OTHER {
                c_type: t.split_once("/").map(|t| t.0).unwrap_or("").to_string(),
                subtype: t.split_once("/").map(|t| t.1).unwrap_or("").to_string(),
                body: resp.bytes().await?,
            },

            None => Element::OTHER {
                body: resp.bytes().await?,
                c_type: "".to_string(),
                subtype: "".to_string(),
            },
        };

        Ok(ele)
    }
}
