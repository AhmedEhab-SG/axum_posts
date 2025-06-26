use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct RootHandler {}

impl RootHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn router(&self) -> Router {
        Router::new().route("/", get(Self::running))
    }

    async fn running() -> Response {
        (
            StatusCode::OK,
            Json(json!({"message": "Welcome to the Axum Store API!"})),
        )
            .into_response()
    }
}
