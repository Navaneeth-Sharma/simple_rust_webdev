use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
// mod always_errors;
mod body_json;
mod body_string;
mod custom_header;
// mod get_json;
mod path_variables;
mod query_params;
mod user_agent;
// mod validate_data;
mod create_tasks;

// use always_errors::always_errors;
use body_json::body_json;
use body_string::body_string;
use custom_header::custom_header;
// use get_json::get_json;
use path_variables::path_variables;
use query_params::query_params;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};
use user_agent::user_agent;
// use validate_data::validate_data;
use create_tasks::create_tasks;

pub fn create_routes(dtbase: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/mirror_body_string", post(body_string))
        .route("/mirror_body_json", post(body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(user_agent))
        .route("/mirror_custom_header", get(custom_header))
        // .route("/always_errors", get(always_errors))
        // .route("/get_json", get(get_json))
        .route("/tasks", post(create_tasks))
        .layer(cors)
        .layer(Extension(dtbase))
    // .route("/validate_data", post(validate_data))
}
