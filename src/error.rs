use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPaasswordLength(usize),
    HashingError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExists,
    UserNoLongerExists,
    TokenNotProvided,
    ExpiredToken,
    PermissionDenied,
    UserNotAuthenticated,
    InvalidHashForamt,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        match self {
            Self::EmptyPassword => "Password cannot be empty".to_string(),
            Self::ExceededMaxPaasswordLength(length) => {
                format!("Password exceeds maximum length of {length}")
            }
            Self::HashingError => "Error hashing password".to_string(),
            Self::InvalidToken => "Invalid token".to_string(),
            Self::ServerError => "Internal server error".to_string(),
            Self::WrongCredentials => "Wrong credentials".to_string(),
            Self::EmailExists => "Email already exists".to_string(),
            Self::UserNoLongerExists => "User no longer exists".to_string(),
            Self::TokenNotProvided => "Token not provided".to_string(),
            Self::ExpiredToken => "Token has expired".to_string(),
            Self::PermissionDenied => "Permission denied".to_string(),
            Self::UserNotAuthenticated => "User not authenticated".to_string(),
            Self::InvalidHashForamt => "Invalid hash format".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub status: StatusCode,
    pub message: String,
}

impl HttpError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn into_http_response(self) -> Response {
        let error_response = ErrorResponse {
            status: self.status.clone().to_string(),
            message: self.message.clone(),
        };
        (self.status, Json(error_response)).into_response()
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let body = Json(json!({ "error": self.message }));
        (self.status, body).into_response()
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "HttpError {{ status: {}, message: {} }}",
            self.status, self.message
        )
    }
}

impl Error for HttpError {}
