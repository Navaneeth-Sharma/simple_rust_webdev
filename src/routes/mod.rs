use axum::{
    routing::{get, post},
    Router,
};
mod body_json;
mod body_string;
mod path_variables;

use body_json::body_json;
use body_string::body_string;
use path_variables::path_variables;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/mirror_body_string", post(body_string))
        .route("/mirror_body_json", post(body_json))
        .route("/path_variables/:id", get(path_variables))
}
