use axum::{headers::UserAgent, TypedHeader};

pub async fn user_agent(TypedHeader(user_ag): TypedHeader<UserAgent>) -> String {
    user_ag.to_string()
}
