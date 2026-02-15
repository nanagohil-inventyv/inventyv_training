use std::fs;

use crate::model::Article;

// get all articles from the file and store into vector

pub fn load_articles() -> Vec<Article> {
    fs::read_to_string("articles.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
        .unwrap_or_else(|_| vec![])
}

pub fn save_articles(articles: &[Article]) {
    fs::write(
        "articles.json",
        serde_json::to_string_pretty(articles).unwrap(),
    )
    .unwrap();
}
