use anyhow::Result;
use api_entity::{UserResponse, UserWithRole, UsersListResponse};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct UserService {
    pool: Pool<Postgres>,
}

impl UserService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_users(&self, page: i32, per_page: i32) -> Result<UsersListResponse> {
        let offset = (page - 1) * per_page;

        let users_with_roles = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT 
                u.id,
                u.fullname,
                u.email,
                u.phone_number,
                u.role_id,
                r.name as role_name,
                u.created_at,
                u.updated_at
            FROM app_users u
            JOIN app_roles r ON u.role_id = r.id
            ORDER BY u.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM app_users")
            .fetch_one(&self.pool)
            .await?;

        let total_pages = (total.0 as f64 / per_page as f64).ceil() as i32;

        let users: Vec<UserResponse> = users_with_roles
            .into_iter()
            .map(UserResponse::from)
            .collect();

        Ok(UsersListResponse {
            users,
            total: total.0,
            page,
            per_page,
            total_pages,
        })
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<UserResponse>> {
        let user_with_role = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT 
                u.id,
                u.fullname,
                u.email,
                u.phone_number,
                u.role_id,
                r.name as role_name,
                u.created_at,
                u.updated_at
            FROM app_users u
            JOIN app_roles r ON u.role_id = r.id
            WHERE u.id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user_with_role.map(UserResponse::from))
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserResponse>> {
        let user_with_role = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT 
                u.id,
                u.fullname,
                u.email,
                u.phone_number,
                u.role_id,
                r.name as role_name,
                u.created_at,
                u.updated_at
            FROM app_users u
            JOIN app_roles r ON u.role_id = r.id
            WHERE u.email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user_with_role.map(UserResponse::from))
    }
}
