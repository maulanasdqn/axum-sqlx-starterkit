use crate::RoleItemWithPermissionsDto;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserItemDto {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub role_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserItemWithRoleDto {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub role: RoleItemWithPermissionsDto,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
