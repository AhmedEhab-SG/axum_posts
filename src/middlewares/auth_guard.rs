use async_trait::async_trait;
use axum::{Extension, extract::Request, http::header, middleware::Next, response::Response};
use uuid::Uuid;

use crate::{
    AppState, db::users_db::UserExt, error::HttpError, middlewares::Middleware,
    utils::token::TokenClaims,
};

#[derive(Debug, Clone)]
pub struct AuthGuard {}

impl AuthGuard {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Middleware for AuthGuard {
    type Extractor = AppState;

    async fn validate_request(
        &self,
        Extension(app_state): Extension<AppState>,
        mut req: Request,
        next: Next,
    ) -> Result<Response, HttpError> {
        let authorization = req
            .headers()
            .get(header::AUTHORIZATION)
            .ok_or_else(|| HttpError::unauthorized("missing authorization header"))?;

        let token = match authorization.to_str() {
            Ok(value) if value.starts_with("Bearer ") => {
                &value.trim_start_matches("Bearer ").to_string()
            }
            _ => {
                return Err(HttpError::unauthorized(
                    "invalid authorization header format",
                ));
            }
        };

        let claim = TokenClaims::decode(token, app_state.env.jwt_access_token_secert.as_bytes())
            .map_err(|_| HttpError::unauthorized("invalid authorization token"))?
            .validate()
            .map_err(|_| HttpError::unauthorized("authorization token is expired or invalid"))?;

        let user_id = Uuid::parse_str(claim.sub.to_string().as_str())
            .map_err(|_| HttpError::unauthorized("invalid user ID in token"))?;

        let user = app_state
            .db_client
            .get_user(Some(user_id), None, None)
            .await
            .map_err(|_| HttpError::server_error("invaild checking for the user"))?
            .ok_or_else(|| {
                HttpError::unauthorized("user not found or does not have access to this resource")
            })?;

        req.extensions_mut().insert(user);

        Ok(next.run(req).await)
    }
}
