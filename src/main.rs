mod config;
mod crawler;
mod youtube_api;
mod data_structures;
mod utils;

fn main() {
    // main application logic
    // Example: Initialize the crawler and start crawling
    let mut crawler = crawler::Crawler::new();
    crawler.crawl("www.youtube.com");
}
