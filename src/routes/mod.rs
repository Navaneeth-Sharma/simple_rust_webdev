use axum::{
    routing::{get, post},
    Router,
};
mod body_json;
mod body_string;
mod custom_header;
mod path_variables;
mod query_params;
mod user_agent;

use body_json::body_json;
use body_string::body_string;
use custom_header::custom_header;
use path_variables::path_variables;
use query_params::query_params;
use user_agent::user_agent;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/mirror_body_string", post(body_string))
        .route("/mirror_body_json", post(body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(user_agent))
        .route("/mirror_custom_header", get(custom_header))
}
