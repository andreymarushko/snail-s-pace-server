use axum::{routing::get, Router};

pub fn router_pages() -> Router {
    Router::new().route("/", get(|| async { "Hello pages" }))
}

pub fn router_games() -> Router {
    Router::new().route("/", get(|| async { "Hello games" }))
}
