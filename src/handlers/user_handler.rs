
use axum::Json;
use axum::extract::{Path, State};

use crate::{app_state::AppState};
use crate::common::error::AppResult;
use crate::common::response::ApiResponse;
pub struct UserHandler;

pub async fn create_user(State(state): State<AppState>) -> String {
   let result = state.user_service.create_user().await;
   result
}

// pub async fn get_user(State(state): State<AppState>, Path(id): Path<u64>) -> AppResult<String> {
pub async fn get_user(State(state): State<AppState>, Path(id): Path<u64>) -> AppResult<Json<ApiResponse<String>>> {
    tracing::info!("Getting user with ID: {}", id);
    let result = state.user_service.get_user(id).await;
    match result {
        Ok(user_name) => Ok(Json(ApiResponse::success(user_name)))  ,
        Err(e) => Err(e),
    }
}