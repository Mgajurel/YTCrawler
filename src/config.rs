pub struct Config {
    pub api_key: String,
    pub crawling_depth: usize,
}

impl Config {
    pub fn new(api_key: String, crawling_depth: usize) -> Self {
        Config { api_key, crawling_depth }
    }
}
