use std::collections::{HashSet, HashMap};
use std::hash;
use std::hash::Hasher;
use std::cmp::Reverse;

#[derive(Eq, Clone)]
pub struct Url {
    pub url: String,
    pub deep: i32,
}

impl hash::Hash for Url {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state)
    }
}

impl PartialEq for Url {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

pub trait UrlManager {
    fn push_url(&mut self, url: Url) -> bool;

    fn next_url(&mut self) -> Option<&Url>;
}

/// 广度优先的 url 管理器，也就是优先 pop 深度最浅的 url. <br/>
/// 若多次添加同一个 url，会将 deep 值设置为最低的一次.
pub struct BreadthFirstUrlManager {
    max_deep: i32,
    url_map: HashMap<String, Url>,
    pq: priority_queue::PriorityQueue<String, Reverse<i32>>,
}

impl BreadthFirstUrlManager {
    pub fn new(max_deep: i32) -> Self {
        Self {
            max_deep,
            url_map: HashMap::new(),
            pq: priority_queue::PriorityQueue::new(),
        }
    }
}

impl UrlManager for BreadthFirstUrlManager {
    fn push_url(&mut self, url: Url) -> bool {
        match self.url_map.get(&url.url) {
            Some(Url { deep: d, .. }) if url.deep < *d => {
                // 更新最新深度
                self.pq.change_priority(&url.url, Reverse(url.deep));
                self.url_map.insert(url.url.clone(), url);
            }
            None => {
                self.pq.push(url.url.clone(), Reverse(url.deep));
                self.url_map.insert(url.url.clone(), url);
            }
            _ => return false
        };
        true
    }

    fn next_url(&mut self) -> Option<&Url> {
        match self.pq.peek() {
            Some((_, deep)) if deep.0 <= self.max_deep => {
                let (url, _) = self.pq.pop().unwrap();
                self.url_map.get(&url)
            }
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breadth_first_url_manager_test() {
        let mut um = BreadthFirstUrlManager::new(3);
        um.push_url(Url { url: "google".to_string(), deep: 3 });
        um.push_url(Url { url: "bing".to_string(), deep: 2 });
        um.push_url(Url { url: "apple".to_string(), deep: 4 });
        assert_eq!(um.next_url().unwrap().url, "bing");
        assert_eq!(um.next_url().unwrap().url, "google");
        assert!(um.next_url().is_none());
    }
}