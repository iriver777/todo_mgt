use crate::common::error::{AppError, AppResult};
use crate::common::response::PaginatedResponse;
use crate::repositories::user_repository::{User, UserRepository};
use std::sync::Arc;

pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;
pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_EMAIL_LENGTH: usize = 255;

pub struct UserService{
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, name: &str, email: &str) -> AppResult<User> {
        let name = name.trim();
        let email = email.trim();

        if name.is_empty() {
            return Err(AppError::ValidationError("name must not be empty".to_string()));
        }
        if name.chars().count() > MAX_NAME_LENGTH {
            return Err(AppError::ValidationError(format!(
                "name must not exceed {MAX_NAME_LENGTH} characters"
            )));
        }
        if email.is_empty() {
            return Err(AppError::ValidationError("email must not be empty".to_string()));
        }
        if email.chars().count() > MAX_EMAIL_LENGTH {
            return Err(AppError::ValidationError(format!(
                "email must not exceed {MAX_EMAIL_LENGTH} characters"
            )));
        }
        if !looks_like_email(email) {
            return Err(AppError::ValidationError(format!(
                "`{email}` is not a valid email address"
            )));
        }

        tracing::info!("Creating a new user in the service layer");
        self.user_repository.create_user(name, email).await
    }

    pub async fn get_user(&self, id: u64) -> AppResult<User> {
        tracing::info!("Will get user with ID: {} in the service layer", id);
        self.user_repository.get_user(id).await
    }

    pub async fn list_users(&self, page: Option<u32>, page_size: Option<u32>) -> AppResult<PaginatedResponse<User>> {
        let page = page.unwrap_or(1).max(1);
        let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE).clamp(1, MAX_PAGE_SIZE);

        if page_size > MAX_PAGE_SIZE {
            return Err(AppError::ValidationError(
                format!("page_size cannot exceed {}", MAX_PAGE_SIZE)
            ));
        }

        tracing::info!(
            "Listing users in service layer: page={}, page_size={}",
            page, page_size
        );

        let total = self.user_repository.count_users().await?;
        let items = self.user_repository.list_users(page, page_size).await?;

        Ok(PaginatedResponse::new(items, total, page, page_size))
    }
}

fn looks_like_email(email: &str) -> bool {
    let mut parts = email.split('@');
    let (Some(local), Some(domain), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && !email.contains(char::is_whitespace)
}
