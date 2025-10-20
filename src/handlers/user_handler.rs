use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{ApiResponse, CreateUserRequest, UpdateUserRequest, User};
use crate::services::UserService;

#[derive(Clone)]
pub struct UserHandler {
    service: UserService,
}

impl UserHandler {
    pub fn new(service: UserService) -> Self {
        Self { service }
    }

    pub async fn get_all(
        State(handler): State<Self>,
    ) -> Result<Json<ApiResponse<Vec<User>>>, AppError> {
        let users = handler.service.get_all_users().await?;
        Ok(Json(ApiResponse::success("Users retrieved successfully", users)))
    }

    pub async fn get_by_id(
        State(handler): State<Self>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<User>>, AppError> {
        let user = handler.service.get_user_by_id(id).await?;
        Ok(Json(ApiResponse::success("User retrieved successfully", user)))
    }

    pub async fn create(
        State(handler): State<Self>,
        Json(request): Json<CreateUserRequest>,
    ) -> Result<(StatusCode, Json<ApiResponse<User>>), AppError> {
        let user = handler.service.create_user(request).await?;
        Ok((
            StatusCode::CREATED,
            Json(ApiResponse::success("User created successfully", user)),
        ))
    }

    pub async fn update(
        State(handler): State<Self>,
        Path(id): Path<Uuid>,
        Json(request): Json<UpdateUserRequest>,
    ) -> Result<Json<ApiResponse<User>>, AppError> {
        let user = handler.service.update_user(id, request).await?;
        Ok(Json(ApiResponse::success("User updated successfully", user)))
    }

    pub async fn delete(
        State(handler): State<Self>,
        Path(id): Path<Uuid>,
    ) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
        handler.service.delete_user(id).await?;
        Ok((
            StatusCode::OK,
            Json(ApiResponse::success("User deleted successfully", ())),
        ))
    }
}
