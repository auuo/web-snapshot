use std::fs::File;
use std::io::Write;

use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use web_snapshot::{
    BreadthFirstUrlManager, Element, ElementHandler, ErrorHandler, SpiderContext, SpiderError, Url,
};

struct HuaBanHandler {
    path: &'static str,
    idx: i32,
}

impl HuaBanHandler {
    fn new(path: &'static str) -> HuaBanHandler {
        HuaBanHandler { path, idx: 0 }
    }
}

impl ElementHandler for HuaBanHandler {
    fn handle(&mut self, ctx: &mut SpiderContext, url: &Url, ele: &Element) -> anyhow::Result<()> {
        lazy_static! {
            static ref PINS_RE: Regex = Regex::new(r#"app.page\["pins"\] = (\[\{.*\}\])"#).unwrap();
        }

        match ele {
            Element::HTML(s) => {
                if let Some(cap) = PINS_RE.captures(s) {
                    let data: Value = serde_json::from_str(&cap[1])?;
                    if let Value::Array(pins) = data {
                        for pin in pins {
                            if let Value::String(key) = &pin["file"]["key"] {
                                ctx.push_url(Url {
                                    url: format!("https://hbimg.huabanimg.com/{}", key),
                                    deep: url.deep + 1,
                                });
                            }
                        }
                    }
                } else {
                    println!("can not extract pins: {}", s);
                }
            }
            Element::IMAGE { body, subtype } => {
                self.idx += 1;

                let mut output = File::create(format!("{}/{}.{}", self.path, self.idx, subtype))?;
                output.write_all(body)?;

                println!("download image success, url: {:?}", url);
            }
            _ => {}
        }

        Ok(())
    }
}

struct PrintErrorHandler {}

impl ErrorHandler for PrintErrorHandler {
    fn handle(&mut self, _ctx: &mut SpiderContext, url: &Url, e: &SpiderError) {
        println!("An error occurred, url: {}, err: {:#?}", url.url, e)
    }
}

fn main() {
    let save_path = "C:/Users/ashley/Desktop/pins";

    let url_manager = BreadthFirstUrlManager::new(2);
    let handlers: Vec<Box<dyn ElementHandler>> = vec![Box::new(HuaBanHandler::new(save_path))];
    let err_handlers: Vec<Box<dyn ErrorHandler>> = vec![Box::new(PrintErrorHandler {})];

    let mut sc = SpiderContext::new(url_manager, handlers, err_handlers);

    sc.push_url(web_snapshot::Url {
        url: "https://huaban.com/discovery/beauty/".to_string(),
        deep: 0,
    });
    sc.run();
}
