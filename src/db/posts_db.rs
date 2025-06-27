use async_trait::async_trait;
use sqlx::{Error as SqlxError, query, query_as, query_scalar};
use uuid::Uuid;

use crate::{db::DBClient, dtos::post_dto::Post};

#[async_trait]
pub trait PostExt {
    async fn get_post_by_id(&self, id: Uuid) -> Result<Option<Post>, SqlxError>;

    async fn get_posts(&self, page: usize, limit: usize) -> Result<Vec<Post>, SqlxError>;

    async fn get_posts_by_user_id(
        &self,
        user_id: Uuid,
        page: usize,
        limit: usize,
    ) -> Result<Vec<Post>, SqlxError>;

    async fn get_posts_count(&self) -> Result<i64, SqlxError>;

    async fn create_post(
        &self,
        user_id: Uuid,
        title: String,
        body: String,
    ) -> Result<Post, SqlxError>;

    async fn update_post(
        &self,
        id: Uuid,
        title: Option<String>,
        body: Option<String>,
    ) -> Result<Option<Post>, SqlxError>;

    async fn delete_post(&self, id: Uuid) -> Result<bool, SqlxError>;
}

#[async_trait]
impl PostExt for DBClient {
    async fn get_post_by_id(&self, id: Uuid) -> Result<Option<Post>, SqlxError> {
        query_as::<_, Post>(
            r#"
            SELECT * FROM posts
            WHERE id = $1
           "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_posts(&self, page: usize, limit: usize) -> Result<Vec<Post>, SqlxError> {
        query_as::<_, Post>(
            r#"
            SELECT * FROM posts 
            ORDER BY created_at DESC 
            LIMIT $1 
            OFFSET $2
            "#,
        )
        .bind(limit as i64)
        .bind((page * limit) as i64)
        .fetch_all(&self.pool)
        .await
    }

    async fn get_posts_by_user_id(
        &self,
        user_id: Uuid,
        page: usize,
        limit: usize,
    ) -> Result<Vec<Post>, SqlxError> {
        query_as::<_, Post>(
            r#"
            SELECT * FROM posts 
            WHERE user_id = $1 
            ORDER BY created_at DESC 
            LIMIT $2 
            OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit as i64)
        .bind((page * limit) as i64)
        .fetch_all(&self.pool)
        .await
    }

    async fn get_posts_count(&self) -> Result<i64, SqlxError> {
        query_scalar(
            r#"
            SELECT COUNT(*) FROM posts
            "#,
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn create_post(
        &self,
        user_id: Uuid,
        title: String,
        body: String,
    ) -> Result<Post, SqlxError> {
        query_as::<_, Post>(
            r#"
            INSERT INTO posts (user_id, title, body)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(title)
        .bind(body)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_post(
        &self,
        id: Uuid,
        title: Option<String>,
        body: Option<String>,
    ) -> Result<Option<Post>, SqlxError> {
        query_as::<_, Post>(
            r#"
            UPDATE posts
            SET 
                title = COALESCE($2, title), 
                body = COALESCE($3, body)
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(title)
        .bind(body)
        .fetch_optional(&self.pool)
        .await
    }

    async fn delete_post(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = query!(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
