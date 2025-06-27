use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    db::{DBClient, posts_db::PostExt},
    error::HttpError,
};

#[derive(Debug, Clone)]
pub struct PostsService {
    db_client: DBClient,
}

impl PostsService {
    pub fn new(db_client: DBClient) -> Self {
        Self { db_client }
    }

    pub async fn get_post(&self, id: Uuid) -> Result<Response, HttpError> {
        let post = self
            .db_client
            .get_post_by_id(id)
            .await
            .map_err(|_| HttpError::server_error("failed to get user"))?
            .ok_or_else(|| HttpError::not_found(format!("user with id: {id} not found")))?;

        let body = Json(json!({
           "post" :post,
        }));

        Ok((StatusCode::OK, body).into_response())
    }

    pub async fn get_posts(&self, page: usize, limit: usize) -> Result<Response, HttpError> {
        let posts = self
            .db_client
            .get_posts(page, limit)
            .await
            .map_err(|_| HttpError::server_error("failed to get posts"))?;

        let post_count = self
            .db_client
            .get_posts_count()
            .await
            .map_err(|_| HttpError::server_error("failed to get post count"))?;

        let body = Json(json!({
            "posts": posts,
            "total": post_count,
        }));

        Ok((StatusCode::OK, body).into_response())
    }

    pub async fn create_post(
        &self,
        user_id: Uuid,
        title: String,
        body: String,
    ) -> Result<Response, HttpError> {
        self.db_client
            .create_post(user_id, title, body)
            .await
            .map_err(|_| HttpError::server_error("failed to create post"))?;

        Ok((StatusCode::CREATED).into_response())
    }

    pub async fn update_post(
        &self,
        id: Uuid,
        title: Option<String>,
        body: Option<String>,
    ) -> Result<Response, HttpError> {
        self.db_client
            .update_post(id, title, body)
            .await
            .map_err(|_| HttpError::server_error("failed to update post"))?;

        Ok((StatusCode::NO_CONTENT).into_response())
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<Response, HttpError> {
        self.db_client
            .delete_post(id)
            .await
            .map_err(|_| HttpError::server_error("failed to delete post"))?;

        Ok((StatusCode::NO_CONTENT).into_response())
    }
}
