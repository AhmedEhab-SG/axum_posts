use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Type)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePostDto {
    pub user_id: Uuid,

    #[validate(length(min = 5, message = "title must be at least 5 characters long"))]
    pub title: String,

    #[validate(length(min = 20, message = "body must be at least 20 characters long"))]
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePostDto {
    #[validate(length(min = 5, message = "title must be at least 5 characters long"))]
    pub title: Option<String>,

    #[validate(length(min = 20, message = "body must be at least 20 characters long"))]
    pub body: Option<String>,
}
