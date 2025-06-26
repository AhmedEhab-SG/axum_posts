use async_trait::async_trait;
use axum::{Extension, extract::Request, middleware::Next, response::Response};

use crate::{
    dtos::user_dto::{User, UserRole},
    error::HttpError,
    middlewares::Middleware,
};

#[derive(Debug, Clone)]
pub struct RolesGuard {
    roles: Vec<UserRole>,
}

impl RolesGuard {
    pub fn new(roles: Vec<UserRole>) -> Self {
        Self { roles }
    }
}

#[async_trait]
impl Middleware for RolesGuard {
    type Extractor = User;

    async fn validate_request(
        &self,
        Extension(user): Extension<User>,
        req: Request,
        next: Next,
    ) -> Result<Response, HttpError> {
        if !self.roles.contains(&user.role) {
            return Err(HttpError::unauthorized(
                "user does not have the required role",
            ));
        }

        Ok(next.run(req).await)
    }
}
