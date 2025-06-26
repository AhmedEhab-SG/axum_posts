use async_trait::async_trait;
use axum::{
    Extension,
    extract::{FromRequestParts, Path, Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    dtos::user_dto::{User, UserRole},
    error::HttpError,
    middlewares::Middleware,
};

pub struct SelfGuard {
    roles: Vec<UserRole>,
}

impl SelfGuard {
    pub fn new(roles: Vec<UserRole>) -> Self {
        Self { roles }
    }
}

#[async_trait]
impl Middleware for SelfGuard {
    type Extractor = User;

    async fn validate_request(
        &self,
        user: Extension<User>,
        mut req: Request,
        next: Next,
    ) -> Result<Response, HttpError> {
        let (mut parts, body) = req.into_parts();

        let Path(id): Path<String> = Path::from_request_parts(&mut parts, &())
            .await
            .map_err(|_| HttpError::bad_request("Missing or invalid `id` param"))?;

        let uuid = Uuid::parse_str(&id)
            .map_err(|_| HttpError::bad_request("Invalid UUID format for `id` param"))?;

        req = Request::from_parts(parts, body);

        if user.id == uuid {
            return Ok(next.run(req).await);
        }

        if !self.roles.contains(&user.role) {
            return Err(HttpError::unauthorized(
                "User does not have the required role",
            ));
        }

        Ok(next.run(req).await)
    }
}
