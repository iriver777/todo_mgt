use serde::Serialize;
use sqlx::{FromRow, PgPool};
use crate::common::error::{AppError, AppResult};

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub struct UserRepository{
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, name: &str, email: &str) -> AppResult<User> {
        tracing::info!("Creating user `{}` in the repository layer", name);
        let result = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        )
        .bind(name)
        .bind(email)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(user) => Ok(user),
            Err(sqlx::Error::Database(ref db_err)) if db_err.code().as_deref() == Some("23505") => {
                Err(AppError::Conflict(format!("email `{email}` is already taken")))
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn get_user(&self, id: u64) -> AppResult<User> {
        tracing::info!("Getting user with ID: {} in the repository layer", id);
        let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = $1")
            .bind(id as i32)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::UserNotFound)?;
        Ok(user)
    }

    pub async fn count_users(&self) -> AppResult<i64> {
        tracing::info!("Counting total users in the repository layer");
        let total: Option<i64> = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_optional(&self.pool)
            .await?;
        Ok(total.unwrap_or(0))
    }

    pub async fn list_users(&self, page: u32, page_size: u32) -> AppResult<Vec<User>> {
        tracing::info!("Listing users: page={}, page_size={} in the repository layer", page, page_size);
        let offset = (page.saturating_sub(1)) as i64 * page_size as i64;
        let users = sqlx::query_as::<_, User>(
            "SELECT id, name, email FROM users ORDER BY id ASC LIMIT $1 OFFSET $2"
        )
            .bind(page_size as i64)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }
}
