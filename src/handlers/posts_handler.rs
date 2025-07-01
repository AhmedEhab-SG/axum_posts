use axum::{
    Extension, Json, Router,
    extract::{Path, Query},
    middleware,
    response::Response,
    routing::{delete, get, patch, post},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    dtos::{
        QueryRangeDto,
        post_dto::{CreatePostDto, UpdatePostDto},
        user_dto::{User, UserRole},
    },
    error::HttpError,
    middlewares::{Middleware, auth_guard::AuthGuard, self_guard::SelfGuard},
    services::posts_service::PostsService,
};

#[derive(Debug, Clone)]
pub struct PostsHandler {
    app_state: AppState,
}

impl PostsHandler {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }

    pub fn router(&self, posts_service: PostsService) -> Router {
        Router::new()
            .route("/user/{id}", get(Self::get_posts_by_user_id))
            .route("/{id}", get(Self::get_post))
            .route("/", get(Self::get_posts))
            .route(
                "/",
                post(Self::create_post).layer(middleware::from_fn(async |state, req, next| {
                    AuthGuard::new().validate_request(state, req, next).await
                })),
            )
            .route(
                "/{id}",
                patch(Self::update_post)
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
                "/{id}",
                delete(Self::delete_post)
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
            .layer(Extension(posts_service))
    }

    async fn get_post(
        Extension(posts_service): Extension<PostsService>,
        Path(id): Path<String>,
    ) -> Result<Response, HttpError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| HttpError::bad_request("Invalid UUID format for `id` param"))?;

        posts_service.get_post(uuid).await
    }

    async fn get_posts_by_user_id(
        Extension(posts_service): Extension<PostsService>,
        Path(id): Path<String>,
        Query(query_params): Query<QueryRangeDto>,
    ) -> Result<Response, HttpError> {
        query_params
            .validate()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        let uuid = Uuid::parse_str(&id)
            .map_err(|_| HttpError::bad_request("Invalid UUID format for `id` param"))?;

        let page = query_params.page.unwrap_or(1);
        let limit = query_params.limit.unwrap_or(10);

        posts_service.get_posts_by_user_id(uuid, page, limit).await
    }

    async fn get_posts(
        Extension(posts_service): Extension<PostsService>,
        Query(query_params): Query<QueryRangeDto>,
    ) -> Result<Response, HttpError> {
        query_params
            .validate()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        let page = query_params.page.unwrap_or(1);
        let limit = query_params.limit.unwrap_or(10);

        posts_service.get_posts(page, limit).await
    }

    async fn create_post(
        Extension(posts_service): Extension<PostsService>,
        Extension(user): Extension<User>,
        Json(post): Json<CreatePostDto>,
    ) -> Result<Response, HttpError> {
        post.validate()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        posts_service
            .create_post(user.id, post.title, post.body)
            .await
    }

    async fn update_post(
        Extension(posts_service): Extension<PostsService>,
        Path(id): Path<String>,
        Json(post): Json<UpdatePostDto>,
    ) -> Result<Response, HttpError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| HttpError::bad_request("Invalid UUID format for `id` param"))?;

        post.validate()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        posts_service.update_post(uuid, post.title, post.body).await
    }

    async fn delete_post(
        Extension(posts_service): Extension<PostsService>,
        Path(id): Path<String>,
    ) -> Result<Response, HttpError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| HttpError::bad_request("Invalid UUID format for `id` param"))?;

        posts_service.delete_post(uuid).await
    }
}
