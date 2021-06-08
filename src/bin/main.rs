use web_snapshot::{SpiderContext, BreadthFirstUrlManager, Element, Url, ElementHandler};

struct PrintHandler {}

impl web_snapshot::ElementHandler for PrintHandler {
    fn handle(&mut self, ctx: &mut SpiderContext, url: &Url, ele: &Element) {
        println!("url: {:?}, {:?}", url, ele);
    }
}

fn main() {
    let url_manager = BreadthFirstUrlManager::new(5);
    let handlers: Vec<Box<dyn ElementHandler>> = vec![Box::new(PrintHandler {})];

    let mut sc = SpiderContext::new(url_manager, handlers, vec![]);

    sc.push_url(web_snapshot::Url { url: "https://www.google.com".to_string(), deep: 0 });
    sc.run();
}