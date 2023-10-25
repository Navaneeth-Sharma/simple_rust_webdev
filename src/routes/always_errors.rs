use axum::{http::StatusCode, response::IntoResponse};

pub async fn always_errors() -> Result<(), StatusCode> {
    // (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_owned()).into_respon.into_responsese()
    Err(StatusCode::IM_A_TEAPOT)
}
