use crate::services::UserService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UsersQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    10
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

pub async fn get_users(
    State(user_service): State<Arc<UserService>>,
    Query(query): Query<UsersQuery>,
) -> Result<Json<api_entity::UsersListResponse>, (StatusCode, Json<ErrorResponse>)> {
    let page = if query.page < 1 { 1 } else { query.page };
    let per_page = if query.per_page < 1 || query.per_page > 100 {
        10
    } else {
        query.per_page
    };

    match user_service.get_users(page, per_page).await {
        Ok(users_response) => Ok(Json(users_response)),
        Err(e) => {
            eprintln!("Error fetching users: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "INTERNAL_SERVER_ERROR".to_string(),
                    message: "Failed to fetch users".to_string(),
                }),
            ))
        }
    }
}

pub async fn get_user_by_id(
    State(user_service): State<Arc<UserService>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<api_entity::UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "USER_NOT_FOUND".to_string(),
                message: format!("User with id {} not found", user_id),
            }),
        )),
        Err(e) => {
            eprintln!("Error fetching user by id {}: {}", user_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "INTERNAL_SERVER_ERROR".to_string(),
                    message: "Failed to fetch user".to_string(),
                }),
            ))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetUserByEmailQuery {
    pub email: String,
}

pub async fn get_user_by_email(
    State(user_service): State<Arc<UserService>>,
    Query(query): Query<GetUserByEmailQuery>,
) -> Result<Json<api_entity::UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    match user_service.get_user_by_email(&query.email).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "USER_NOT_FOUND".to_string(),
                message: format!("User with email {} not found", query.email),
            }),
        )),
        Err(e) => {
            eprintln!("Error fetching user by email {}: {}", query.email, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "INTERNAL_SERVER_ERROR".to_string(),
                    message: "Failed to fetch user".to_string(),
                }),
            ))
        }
    }
}
