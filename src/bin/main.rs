use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::json;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use web_snapshot::{
    BreadthFirstUrlManager, Element, ElementHandler, ErrorHandler, RequestBuilder,
    SpiderApplication, SpiderContext, SpiderError, Url,
};

struct HuaBanHandler {
    path: &'static str,
}

impl HuaBanHandler {
    fn new(path: &'static str) -> HuaBanHandler {
        HuaBanHandler { path }
    }
}

#[async_trait]
impl ElementHandler for HuaBanHandler {
    async fn handle(
        &self,
        ctx: &mut SpiderContext,
        url: &Url,
        ele: &Element,
    ) -> anyhow::Result<()> {
        lazy_static! {
            static ref PINS_RE: Regex = Regex::new(r#"app.page\["pins"\] = (\[\{.*\}\])"#).unwrap();
        }

        match ele {
            Element::HTML(s) => {
                if let Some(cap) = PINS_RE.captures(s) {
                    let data: Value = serde_json::from_str(&cap[1])?;
                    self.handle_pins_array(ctx, &data, url.deep + 1).await;
                } else {
                    println!("can not extract pins: {}", s);
                }
            }
            Element::IMAGE { body, subtype } => {
                let mut output = File::create(format!(
                    "{}/{}.{}",
                    self.path,
                    url.data["file_id"]
                        .as_i64()
                        .map(|i| i.to_string())
                        .unwrap_or(Uuid::new_v4().to_string()),
                    subtype
                ))
                .await?;
                output.write_all(body).await?;

                println!("download image success, url: {:?}", url);
            }
            Element::JSON(json) => {
                let data: Value = serde_json::from_str(json)?;
                self.handle_pins_array(ctx, &data["pins"], url.deep + 1)
                    .await;
            }
            _ => {}
        }

        Ok(())
    }
}

impl HuaBanHandler {
    async fn handle_pins_array(
        &self,
        ctx: &mut SpiderContext,
        data: &serde_json::Value,
        cur_deep: i32,
    ) {
        if let Value::Array(pins) = data {
            let mut max_pin_id = 0i64;

            for pin in pins {
                max_pin_id = max_pin_id.max(pin["pin_id"].as_i64().unwrap_or(0));

                if let Value::String(key) = &pin["file"]["key"] {
                    ctx.push_url(Url::new_with_data(
                        format!("https://hbimg.huabanimg.com/{}", key),
                        cur_deep + 1,
                        json!({"file_id": pin["file_id"].clone()}),
                    ))
                    .await;
                }

                if let Some(pin_id) = pin["pin_id"].as_i64() {
                    self.add_more_pins(ctx, pin_id, cur_deep).await;
                }
            }

            println!("add pins: {:?}", pins);
        }
    }

    async fn add_more_pins(&self, ctx: &mut SpiderContext, pin_id: i64, cur_deep: i32) {
        ctx.push_url(Url::new_with_data(
            format!(
                "https://huaban.com/discovery/beauty/?kqfbzohe&max={}&limit=30&wfl=1",
                pin_id
            ),
            cur_deep + 1,
            json!({
                "http_header": {
                    "X-Request": "JSON",
                    "X-Requested-With": "XMLHttpRequest"
                }
            }),
        ))
        .await;
    }
}

struct PrintErrorHandler {}

#[async_trait]
impl ErrorHandler for PrintErrorHandler {
    async fn handle(&self, _ctx: &mut SpiderContext, url: &Url, e: &SpiderError) {
        println!("An error occurred, url: {}, err: {:#?}", url.url, e)
    }
}

struct HuaBanRequestBuilder {}

#[async_trait]
impl RequestBuilder for HuaBanRequestBuilder {
    async fn build(&self, url: &Url) -> reqwest::Result<reqwest::Response> {
        if let serde_json::Value::Object(headers) = &url.data["http_header"] {
            let client = reqwest::Client::new();
            let mut rb = client.get(&url.url);

            for (k, v) in headers.iter() {
                rb = rb.header(k, v.as_str().unwrap_or(""));
            }

            client.execute(rb.build()?).await
        } else {
            reqwest::get(&url.url).await
        }
    }
}

#[tokio::main]
async fn main() {
    let save_path = "C:/Users/ashley/Desktop/async_pins";

    let url_manager = BreadthFirstUrlManager::new(100);
    let handlers: Vec<Box<dyn ElementHandler>> = vec![Box::new(HuaBanHandler::new(save_path))];
    let err_handlers: Vec<Box<dyn ErrorHandler>> = vec![Box::new(PrintErrorHandler {})];

    let mut app = SpiderApplication::new(
        url_manager,
        handlers,
        err_handlers,
        Some(Box::new(HuaBanRequestBuilder {})),
    );

    app.push_url(Url::new(
        "https://huaban.com/discovery/beauty/".to_string(),
        0,
    ))
    .await;

    app.run().await;
}
