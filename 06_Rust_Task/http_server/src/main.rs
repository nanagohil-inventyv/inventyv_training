pub mod api;
pub mod handler;
pub mod model;
pub mod route;

use std::{
    net::SocketAddr,
    sync::Arc,
};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use crate::model::Article;




pub type SharedState = Arc<RwLock<Vec<Article>>>;

#[tokio::main]
async fn main() {
    let articles = handler::load_articles();

    let state = Arc::new(RwLock::new(articles));
    let app = route::app_routes().with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    // Run Server

    axum::serve(listener, app).await.unwrap();
}
