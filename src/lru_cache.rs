use std::collections::HashMap;

struct LruCache<'a> {
    pub contents_hash_map: HashMap<String, Box<Content>>,
    pub contents_usage: Vec<Box<Content>>
}

struct Content {

}

impl LruCache {
    fn new(fixed_size: u32) -> Self {
        LruCache {
            contents_hash_map: HashMap::new(),
            contents_usage: vec![0, fixed_size]
        }
    }

    fn get(&self, key: String) -> Result<&Box<String>, &'static str> {
        match self.contents_hash_map.get(&key) {
            None => Err("No content for key found"),
            Some(content) => &content
        }
    }

    fn put(&mut self, key: String, content: String) {
        self.contents_hash_map.insert(key, Box::new(content));
    }
}