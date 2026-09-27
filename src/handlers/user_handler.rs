use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::common::error::AppResult;
use crate::common::response::{ApiResponse, PaginatedResponse};
use crate::repositories::user_repository::User;

pub struct UserHandler;

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<(StatusCode, Json<ApiResponse<User>>)> {
    tracing::info!("Creating user: name={}", payload.name);
    let user = state.user_service.create_user(&payload.name, &payload.email).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::success(user))))
}

pub async fn get_user(State(state): State<AppState>, Path(id): Path<u64>) -> AppResult<Json<ApiResponse<User>>> {
    tracing::info!("Getting user with ID: {}", id);
    let user = state.user_service.get_user(id).await?;
    Ok(Json(ApiResponse::success(user)))
}

pub async fn get_users(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
) -> AppResult<Json<ApiResponse<PaginatedResponse<User>>>> {
    tracing::info!("Listing users with query params: page={:?}, page_size={:?}", query.page, query.page_size);
    let result = state.user_service.list_users(query.page, query.page_size).await?;
    Ok(Json(ApiResponse::success(result)))
}
