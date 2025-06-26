use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    db::{DBClient, users::UserExt},
    dtos::user_dto::{UserRequestDto, UserResponseDto, UserRole, UsersResponseDtoList},
    error::HttpError,
    utils::password::PasswordArgon,
};

#[derive(Debug, Clone)]
pub struct UsersService {
    db_client: DBClient,
}

impl UsersService {
    pub fn new(db_client: DBClient) -> Self {
        Self { db_client }
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Response, HttpError> {
        let user = self
            .db_client
            .get_user(Some(id), None, None)
            .await
            .map_err(|_| HttpError::server_error("failed to get user"))?
            .ok_or_else(|| HttpError::not_found(format!("user with id: {id} not found")))?;

        let user_response = UserResponseDto::from_user(&user);

        let body = Json(json!({
            "user": user_response,
        }));

        Ok((StatusCode::OK, body).into_response())
    }

    pub async fn get_users(&self, page: usize, limit: usize) -> Result<Response, HttpError> {
        let users = self
            .db_client
            .get_users(page, limit)
            .await
            .map_err(|_| HttpError::server_error("failed to get users"))?;

        let user_count = self
            .db_client
            .get_user_count()
            .await
            .map_err(|_| HttpError::server_error("failed to get user count"))?;

        let users_response = UsersResponseDtoList::from_users(&users);

        let body = Json(json!({
            "users": users_response.users_list,
            "total": user_count,
            "page": page,
            "limit": limit,
        }));

        Ok((StatusCode::OK, body).into_response())
    }

    pub async fn update_user(&self, id: Uuid, data: UserRequestDto) -> Result<Response, HttpError> {
        let password = match data.password {
            Some(password) => Some(
                PasswordArgon::hash(&password)
                    .map_err(|_| HttpError::server_error("failed to hash password"))?,
            ),
            None => None,
        };

        self.db_client
            .update_user(id, data.email, password)
            .await
            .map_err(|_| HttpError::server_error("failed to update user"))?;

        Ok((StatusCode::NO_CONTENT).into_response())
    }

    pub async fn update_user_role(&self, id: Uuid, role: UserRole) -> Result<Response, HttpError> {
        self.db_client
            .update_user_role(id, role)
            .await
            .map_err(|_| HttpError::server_error("failed to update user role"))?;

        Ok((StatusCode::NO_CONTENT).into_response())
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<Response, HttpError> {
        self.db_client
            .delete_user(id)
            .await
            .map_err(|_| HttpError::server_error("failed to delete user"))?;

        Ok((StatusCode::NO_CONTENT).into_response())
    }
}
