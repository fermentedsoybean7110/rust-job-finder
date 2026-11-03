pub mod api;
pub mod config;
pub mod models;
pub mod routes;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

pub fn app() -> Router {
    Router::new()
        .route("/", get(routes::jobs::index))
        .nest_service("/static", ServeDir::new("static"))
}
