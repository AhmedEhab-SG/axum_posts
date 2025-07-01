use axum::{
    Extension, Json, Router,
    extract::{Path, Query},
    middleware,
    response::Response,
    routing::{delete, get, patch, put},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    dtos::{
        QueryRangeDto,
        user_dto::{UpdateUserDto, UpdateUserRoleDto, UserRole},
    },
    error::HttpError,
    middlewares::{
        Middleware, auth_guard::AuthGuard, roles_guard::RolesGuard, self_guard::SelfGuard,
    },
    services::users_service::UsersService,
};

#[derive(Debug, Clone)]
pub struct UsersHandler {
    app_state: AppState,
}

impl UsersHandler {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }

    pub fn router(&self, users_service: UsersService) -> Router {
        Router::new()
            .route("/{id}", get(Self::get_user))
            .route(
                "/",
                get(Self::get_users)
                    .layer(middleware::from_fn(async |user, req, next| {
                        RolesGuard::new(vec![UserRole::Admin])
                            .validate_request(user, req, next)
                            .await
                    }))
                    .layer(middleware::from_fn(async |state, req, next| {
                        AuthGuard::new().validate_request(state, req, next).await
                    })),
            )
            .route(
                "/{id}",
                patch(Self::update_user)
                    .layer(middleware::from_fn(async |user, req, next| {
                        SelfGuard::new(vec![UserRole::Admin])
                            .validate_request(user, req, next)
                            .await
                    }))
                    .layer(middleware::from_fn(async |state, req, next| {
                        AuthGuard::new().validate_request(state, req, next).await
                    })),
            )
            .route(
                "/role/{id}",
                put(Self::update_user_role)
                    .layer(middleware::from_fn(async |user, req, next| {
                        RolesGuard::new(vec![UserRole::Admin])
                            .validate_request(user, req, next)
                            .await
                    }))
                    .layer(middleware::from_fn(async |state, req, next| {
                        AuthGuard::new().validate_request(state, req, next).await
                    })),
            )
            .route(
                "/{id}",
                delete(Self::delete_user)
                    .layer(middleware::from_fn(async |user, req, next| {
                        SelfGuard::new(vec![UserRole::Admin])
                            .validate_request(user, req, next)
                            .await
                    }))
                    .layer(middleware::from_fn(async |state, req, next| {
                        AuthGuard::new().validate_request(state, req, next).await
                    })),
            )
            .layer(Extension(self.app_state.clone()))
            .layer(Extension(users_service))
    }
    async fn get_user(
        Extension(users_service): Extension<UsersService>,
        Path(id): Path<String>,
    ) -> Result<Response, HttpError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| HttpError::bad_request("Invalid UUID format for `id` param"))?;

        users_service.get_user(uuid).await
    }

    async fn get_users(
        Query(query_params): Query<QueryRangeDto>,
        Extension(users_service): Extension<UsersService>,
    ) -> Result<Response, HttpError> {
        query_params
            .validate()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        let page = query_params.page.unwrap_or(1);
        let limit = query_params.limit.unwrap_or(10);

        users_service.get_users(page, limit).await
    }

    async fn update_user(
        Extension(users_service): Extension<UsersService>,
        Path(id): Path<String>,
        Json(body): Json<UpdateUserDto>,
    ) -> Result<Response, HttpError> {
        body.validate()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        let uuid = Uuid::parse_str(&id)
            .map_err(|_| HttpError::bad_request("Invalid UUID format for `id` param"))?;

        users_service.update_user(uuid, body).await
    }

    async fn update_user_role(
        Extension(users_service): Extension<UsersService>,
        Path(id): Path<String>,
        Json(body): Json<UpdateUserRoleDto>,
    ) -> Result<Response, HttpError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| HttpError::bad_request("Invalid UUID format for `id` param"))?;

        users_service.update_user_role(uuid, body).await
    }

    async fn delete_user(
        Extension(users_service): Extension<UsersService>,
        Path(id): Path<String>,
    ) -> Result<Response, HttpError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| HttpError::bad_request("Invalid UUID format for `id` param"))?;

        users_service.delete_user(uuid).await
    }
}
