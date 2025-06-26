pub mod auth_guard;
pub mod roles_guard;
pub mod self_guard;

use async_trait::async_trait;
use axum::{Extension, extract::Request, middleware::Next, response::IntoResponse};

use crate::error::HttpError;

#[async_trait]
pub trait Middleware {
    type Extractor: Send + Sync;

    async fn validate_request(
        &self,
        Extension(Ext): Extension<Self::Extractor>,
        mut req: Request,
        next: Next,
    ) -> Result<impl IntoResponse, HttpError>;
}
