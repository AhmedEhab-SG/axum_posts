use async_trait::async_trait;
use sqlx::{Error as SqlxError, query, query_as};
use uuid::Uuid;

use crate::{
    db::DBClient,
    dtos::user_dto::{User, UserRole},
};

#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, SqlxError>;

    async fn get_users(&self, page: usize, limit: usize) -> Result<Vec<User>, SqlxError>;

    async fn get_user_count(&self) -> Result<i64, SqlxError>;

    async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, SqlxError>;

    async fn update_user(
        &self,
        id: Uuid,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<Option<User>, SqlxError>;

    async fn update_user_role(&self, id: Uuid, role: UserRole) -> Result<User, SqlxError>;

    async fn delete_user(&self, id: Uuid) -> Result<bool, SqlxError>;
}

#[async_trait]
impl UserExt for DBClient {
    async fn get_user(
        &self,
        id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, SqlxError> {
        match (id, name, email) {
            (Some(id), _, _) => {
                query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await
            }
            (_, Some(name), _) => {
                query_as::<_, User>("SELECT * FROM users WHERE name = $1")
                    .bind(name)
                    .fetch_optional(&self.pool)
                    .await
            }
            (_, _, Some(email)) => {
                query_as::<_, User>("SELECT * FROM users WHERE email = $1")
                    .bind(email)
                    .fetch_optional(&self.pool)
                    .await
            }
            _ => Ok(None),
        }
    }

    async fn get_users(&self, page: usize, limit: usize) -> Result<Vec<User>, SqlxError> {
        query_as::<_, User>(
            r#"
            SELECT * FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit as i64)
        .bind(((page - 1) * limit) as i64)
        .fetch_all(&self.pool)
        .await
    }

    async fn get_user_count(&self) -> Result<i64, SqlxError> {
        Ok(query_as::<_, (i64,)>(
            r#"
            SELECT COUNT(*) FROM users
            "#,
        )
        .fetch_one(&self.pool)
        .await?
        .0)
    }

    async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, SqlxError> {
        query_as::<_, User>(
            r#"
            INSERT INTO users (name, email, password)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(name)
        .bind(email)
        .bind(password)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_user(
        &self,
        id: Uuid,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<Option<User>, SqlxError> {
        Ok(query_as::<_, User>(
            r#"
            UPDATE users
            SET
                email = COALESCE($1, email),
                password = COALESCE($2, password),
                updated_at = NOW()
            WHERE id = $3
            RETURNING *
            "#,
        )
        .bind(email)
        .bind(password)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn update_user_role(&self, id: Uuid, role: UserRole) -> Result<User, SqlxError> {
        query_as::<_, User>(
            r#"
            UPDATE users
            SET role = $1
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(role)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_user(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
