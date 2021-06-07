use web_snapshot::{SpiderContext, BreadthFirstUrlManager, Element, Url};

struct PrintHandler {}

impl web_snapshot::ElementHandler for PrintHandler {
    fn handle(&mut self, ctx: &mut SpiderContext, url: &Url, ele: &Element) {
        println!("url: {:?}, {:?}", url, ele);

        ctx.push_url(Url {
            url: format!("{}-{}", url.url, url.deep + 1),
            deep: url.deep + 1,
        });
    }
}

fn main() {
    let url_manager = BreadthFirstUrlManager::new(5);
    let mut sc = SpiderContext::new(url_manager, vec![
        Box::new(PrintHandler {})
    ]);

    sc.push_url(web_snapshot::Url { url: "google".to_string(), deep: 0 });
    sc.run();
}