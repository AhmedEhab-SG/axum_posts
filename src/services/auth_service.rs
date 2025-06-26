use time::Duration;

use axum::{
    Json,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::Cookie;
use serde_json::json;

use crate::{
    AppState,
    db::{DBClient, users::UserExt},
    dtos::{
        auth_dto::{LoginDto, RegisterDto},
        user_dto::UserResponseDto,
    },
    error::HttpError,
    utils::{config::Env, password::PasswordArgon, token::TokenClaims},
};

#[derive(Debug, Clone)]
pub struct AuthService {
    db_client: DBClient,
    env: Env,
}

impl AuthService {
    pub fn new(app_state: AppState) -> Self {
        Self {
            db_client: app_state.db_client,
            env: app_state.env,
        }
    }

    pub fn refresh(&self, cookie: Cookie) -> Result<Response, HttpError> {
        if cookie.name() != "refresh_token" {
            return Err(HttpError::bad_request("refresh token not found"));
        }

        let refresh_token =
            TokenClaims::decode(cookie.value(), self.env.jwt_refresh_token_secert.as_bytes())
                .map_err(|_| HttpError::unauthorized("invalid refresh token"))?
                .validate()
                .map_err(|_| HttpError::unauthorized("expired refresh token"))?;

        let access_token = TokenClaims::encode(
            &refresh_token.sub.to_string(),
            self.env.jwt_access_token_secert.as_bytes(),
            self.env.jwt_access_token_expires,
        )
        .map_err(|_| HttpError::server_error("failed to generate access token"))?;

        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {access_token}"))
            .map_err(|_| HttpError::server_error("failed to set access token header"))?;
        headers.insert(header::AUTHORIZATION, auth_value);

        Ok((StatusCode::OK, headers).into_response())
    }

    pub async fn register(&self, data: RegisterDto) -> Result<StatusCode, HttpError> {
        let found_user = self
            .db_client
            .get_user(None, None, Some(&data.email))
            .await
            .map_err(|_| HttpError::server_error("failed to check existing user"))?;

        if found_user.is_some() {
            return Err(HttpError::bad_request("user already exists"));
        }

        let password = PasswordArgon::hash(&data.password)
            .map_err(|_| HttpError::server_error("failed to hash password"))?;

        self.db_client
            .create_user(data.name, data.email, password)
            .await
            .map_err(|_| HttpError::server_error("failed to create user"))?;

        Ok(StatusCode::CREATED)
    }

    pub async fn login(&self, data: LoginDto) -> Result<Response, HttpError> {
        let user = self
            .db_client
            .get_user(None, None, Some(&data.email))
            .await
            .map_err(|_| HttpError::unauthorized("invalid credentials"))?
            .ok_or_else(|| HttpError::unauthorized("invalid credentials"))?;

        let password_matches = PasswordArgon::compare(&data.password, &user.password)
            .map_err(|_| HttpError::server_error("password comparison failed"))?;

        if !password_matches {
            return Err(HttpError::unauthorized("invalid credentials"));
        }

        let user_response = UserResponseDto::from_user(&user);

        let access_token = TokenClaims::encode(
            &user.id.to_string(),
            self.env.jwt_access_token_secert.as_bytes(),
            self.env.jwt_access_token_expires,
        )
        .map_err(|_| HttpError::server_error("failed to generate access token"))?;

        let refresh_token = TokenClaims::encode(
            &user.id.to_string(),
            self.env.jwt_refresh_token_secert.as_bytes(),
            self.env.jwt_refresh_token_expires,
        )
        .map_err(|_| HttpError::server_error("failed to generate refresh token"))?;

        let cookie = Cookie::build(("refresh_token", refresh_token))
            .http_only(true)
            //.secure(true)
            .path("/")
            .max_age(Duration::seconds(self.env.jwt_refresh_token_expires))
            .build();

        let mut headers = HeaderMap::new();

        let auth_value = HeaderValue::from_str(&format!("Bearer {access_token}"))
            .map_err(|_| HttpError::server_error("failed to set access token header"))?;
        headers.insert(header::AUTHORIZATION, auth_value);

        let cookie_value = HeaderValue::from_str(&cookie.to_string())
            .map_err(|_| HttpError::server_error("failed to set refresh token cookie"))?;
        headers.append(header::SET_COOKIE, cookie_value);

        let body = Json(json!({
            "message": "Login successful",
            "user": user_response,
        }));

        Ok((StatusCode::OK, headers, body).into_response())
    }

    pub fn logout(&self, cookie: Cookie) -> Result<Response, HttpError> {
        if cookie.name() != "refresh_token" {
            return Err(HttpError::bad_request("refresh token not found"));
        }

        let cookie_empty = Cookie::build(("refresh_token", ""))
            .http_only(true)
            //.secure(true)
            .path("/")
            .build();

        let mut headers = HeaderMap::new();
        let cookie_value = HeaderValue::from_str(&cookie_empty.to_string())
            .map_err(|_| HttpError::server_error("failed to set empty refresh token cookie"))?;

        headers.append(header::SET_COOKIE, cookie_value);

        Ok((StatusCode::OK, headers).into_response())
    }
}
