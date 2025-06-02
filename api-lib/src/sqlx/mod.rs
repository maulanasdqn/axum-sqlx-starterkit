use super::Env;
use anyhow::{Result, anyhow};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init_db() -> Result<Pool<Postgres>> {
    let env = Env::new();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env.database_url)
        .await;
    match pool {
        Ok(pool) => Ok(pool),
        Err(err) => Err(anyhow!("Failed to connect to database: {}", err)),
    }
}
