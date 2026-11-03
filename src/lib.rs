pub mod api;
pub mod config;
pub mod models;
pub mod routes;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::api::JobsClient;

#[derive(Clone)]
pub struct AppState {
    pub client: JobsClient,
}

pub fn app(client: JobsClient) -> Router {
    Router::new()
        .route("/", get(routes::jobs::index))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(AppState { client })
}
