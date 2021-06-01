pub trait UrlManager {
    fn push_url(&mut self, url: String) -> bool;

    fn next_url(&mut self) -> Option<String>;
}

pub struct BreadthFirstUrlManager {
    max_deep: i32,
    urls: Vec<String>,
}

impl BreadthFirstUrlManager {
    pub fn new(max_deep: i32) -> Self {
        Self {
            max_deep,
            urls: vec![],
        }
    }
}

impl UrlManager for BreadthFirstUrlManager {
    fn push_url(&mut self, url: String) -> bool {
        self.urls.push(url);
        // todo check if already exist
        return true;
    }

    fn next_url(&mut self) -> Option<String> {
        self.urls.pop()
    }
}