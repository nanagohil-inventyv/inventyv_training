use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{SharedState, api::*};

pub fn app_routes() -> Router<SharedState> {
    Router::new()
        .route("/articles", post(create_article).get(get_articles))
        .route(
            "/article/{id}",
            get(get_article).delete(delete_article).put(update_article),
        )
        .route("/article/comment/{id}", post(add_comment))
}
