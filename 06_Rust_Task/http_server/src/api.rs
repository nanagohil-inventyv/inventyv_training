use crate::{SharedState, handler::save_articles, model::*};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use uuid::Uuid;

/// Get all articles
pub async fn get_articles(State(state): State<SharedState>) -> Json<Vec<Article>> {
    let articles = state.read().await;
    Json(articles.clone())
}

/// Get single article
pub async fn get_article(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Article>, StatusCode> {
    let articles = state.read().await;

    articles
        .iter()
        .find(|a| a.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Create article
pub async fn create_article(
    State(state): State<SharedState>,
    Json(mut article): Json<Article>,
) -> (StatusCode, Json<Article>) {
    article.id = Uuid::new_v4().to_string();
    article.comments = vec![];

    let mut articles = state.write().await;
    articles.push(article.clone());

    save_articles(&articles);

    (StatusCode::CREATED, Json(article))
}

/// Update article
pub async fn update_article(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(updated): Json<Article>,
) -> StatusCode {
    let mut articles = state.write().await;

    if let Some(article) = articles.iter_mut().find(|a| a.id == id) {
        article.title = updated.title;
        article.content = updated.content;
        article.comments = updated.comments;

        save_articles(&articles);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Delete article
pub async fn delete_article(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> StatusCode {
    let mut articles = state.write().await;

    let len_before = articles.len();
    articles.retain(|a| a.id != id);

    if articles.len() != len_before {
        save_articles(&articles);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Add comment
pub async fn add_comment(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(mut comment): Json<Comment>,
) -> Result<(StatusCode, Json<Comment>), StatusCode> {
    comment.id = Uuid::new_v4().to_string();

    let mut articles = state.write().await;

    if let Some(article) = articles.iter_mut().find(|a| a.id == id) {
        article.comments.push(comment.clone());

        save_articles(&articles);

        Ok((StatusCode::CREATED, Json(comment)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
