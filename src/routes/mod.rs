use axum::{
    routing::{get, post},
    Router,
};
mod body_string;
use body_string::body_string;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/mirror_body_string", post(body_string))
}
