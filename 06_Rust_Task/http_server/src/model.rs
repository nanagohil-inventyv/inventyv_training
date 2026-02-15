use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub content: String,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    #[serde(default)]
    pub id: String,
    pub author: String,
    pub text: String,
}
