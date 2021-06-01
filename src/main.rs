use web_snapshot::{SpiderContext, BreadthFirstUrlManager};

fn main() {
    let url_manager = BreadthFirstUrlManager::new(5);
    let mut sc = SpiderContext::new(url_manager, vec![]);
    sc.run();
}