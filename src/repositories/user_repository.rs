use sqlx::PgPool;
use crate::common::error::{AppError, AppResult};

pub struct UserRepository{
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self) -> String {
        "Creating a new user in the repository layer".to_string()
    }

    pub async fn get_user(&self, id: u64) -> AppResult<String> {
        tracing::info!("Getting user with ID: {} in the repository layer", id);
        let name = sqlx::query_scalar("SELECT name FROM users WHERE id = $1")
            .bind(id as i64) // Assuming id is stored as BIGINT in the database
            .fetch_optional(&self.pool)
            .await?;
        return name.ok_or(AppError::UserNotFound);
    }
}