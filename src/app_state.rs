use std::sync::Arc;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    // You can add shared state here, such as database connections, configuration, etc.
    pub user_service: Arc<UserService>,
}