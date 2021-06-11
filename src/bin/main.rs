use web_snapshot::{SpiderContext, BreadthFirstUrlManager, Element, Url, ElementHandler, SpiderError, ErrorHandler};

struct HuaBanHandler {}

impl ElementHandler for HuaBanHandler {
    fn handle(&mut self, ctx: &mut SpiderContext, url: &Url, ele: &Element) {
        // todo find image url
        println!("url: {:?}, {:?}", url, ele);
    }
}

struct PrintErrorHandler {}

impl ErrorHandler for PrintErrorHandler {
    fn handle(&mut self, ctx: &mut SpiderContext, url: &Url, e: &SpiderError) {
        println!("An error occurred, url: {}, err: {:#?}", url.url, e)
    }
}

fn main() {
    let url_manager = BreadthFirstUrlManager::new(2);
    let handlers: Vec<Box<dyn ElementHandler>> = vec![Box::new(HuaBanHandler {})];
    let err_handlers: Vec<Box<dyn ErrorHandler>> = vec![Box::new(PrintErrorHandler {})];

    let mut sc = SpiderContext::new(url_manager, handlers, err_handlers);

    sc.push_url(web_snapshot::Url { url: "https://huaban.com/discovery/beauty/".to_string(), deep: 0 });
    sc.run();
}