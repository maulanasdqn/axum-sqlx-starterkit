use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
    #[serde(skip_serializing)]
    pub password: String,
    pub role_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserWithRole {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub role_id: Uuid,
    pub role_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub role: RoleInfo,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleInfo {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersListResponse {
    pub users: Vec<UserResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

impl From<UserWithRole> for UserResponse {
    fn from(user_with_role: UserWithRole) -> Self {
        Self {
            id: user_with_role.id,
            fullname: user_with_role.fullname,
            email: user_with_role.email,
            phone_number: user_with_role.phone_number,
            role: RoleInfo {
                id: user_with_role.role_id,
                name: user_with_role.role_name,
            },
            created_at: DateTime::from_naive_utc_and_offset(user_with_role.created_at, Utc),
            updated_at: DateTime::from_naive_utc_and_offset(user_with_role.updated_at, Utc),
        }
    }
}
