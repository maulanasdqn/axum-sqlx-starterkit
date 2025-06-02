use api_core::{get_user_by_email, get_user_by_id, get_users, UserService};
use api_lib::axum_init;
use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Axum server is running!",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

fn apps(init_db: Pool<Postgres>) -> Router {
    let user_service = Arc::new(UserService::new(init_db));

    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user_by_id))
        .route("/users/by-email", get(get_user_by_email))
        .with_state(user_service)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    axum_init(|init_db| async move { apps(init_db) }).await;
}
