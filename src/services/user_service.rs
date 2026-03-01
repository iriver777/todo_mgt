
use crate::{common::error::AppResult, repositories::user_repository::UserRepository};
use std::sync::Arc;
pub struct UserService{
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }
    pub async fn create_user(&self) -> String {
        println!("Creating a new user in the service layer");
        let ret = self.user_repository.create_user().await;
        ret
    }

    pub async fn get_user(&self,id: u64) -> AppResult<String> {
        tracing::info!("Will  get user with ID: {} in the service layer", id);
        let ret = self.user_repository.get_user(id).await?;
        Ok(format!("Got user with ID: {} and result: {}", id, ret))
    }
}

