use axum::{
    Extension, Json, Router,
    http::{HeaderMap, StatusCode, header},
    response::Response,
    routing::{delete, get, post},
};
use axum_extra::extract::cookie::Cookie;
use validator::Validate;

use crate::{
    dtos::auth_dto::{LoginDto, RegisterDto},
    error::HttpError,
    services::auth_service::AuthService,
};

#[derive(Debug, Clone)]
pub struct AuthHandler {}

impl AuthHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn router(&self, auth_service: AuthService) -> Router {
        Router::new()
            .route("/refresh", get(Self::refresh))
            .route("/login", post(Self::login))
            .route("/register", post(Self::register))
            .route("/logout", delete(Self::logout))
            .layer(Extension(auth_service))
    }

    async fn refresh(
        Extension(auth_service): Extension<AuthService>,
        headers: HeaderMap,
    ) -> Result<Response, HttpError> {
        match headers.get(header::COOKIE) {
            Some(cookie_header) => {
                let cookie = Cookie::parse(cookie_header.to_str().unwrap_or(""))
                    .map_err(|_| HttpError::bad_request("invalid cookie"))?;

                auth_service.refresh(cookie)
            }
            None => Err(HttpError::unauthorized("missing authentication cookie")),
        }
    }

    async fn register(
        Extension(auth_service): Extension<AuthService>,
        Json(body): Json<RegisterDto>,
    ) -> Result<StatusCode, HttpError> {
        body.validate()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        auth_service.register(body).await
    }

    async fn login(
        Extension(auth_service): Extension<AuthService>,
        Json(body): Json<LoginDto>,
    ) -> Result<Response, HttpError> {
        body.validate()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        auth_service.login(body).await
    }

    async fn logout(
        Extension(auth_service): Extension<AuthService>,
        headers: HeaderMap,
    ) -> Result<Response, HttpError> {
        match headers.get(header::COOKIE) {
            Some(cookie_header) => {
                let cookie = Cookie::parse(cookie_header.to_str().unwrap_or(""))
                    .map_err(|_| HttpError::bad_request("invalid cookie"))?;

                auth_service.logout(cookie)
            }
            None => Err(HttpError::unauthorized("missing authentication cookie")),
        }
    }
}
