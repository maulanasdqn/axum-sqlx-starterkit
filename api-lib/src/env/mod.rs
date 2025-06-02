use std::env;

pub struct Env {
    pub port: u16,
    pub database_url: String,
    pub rust_env: String,
}

impl Env {
    pub fn new() -> Self {
        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:@localhost:5432/postgres".to_string()),
            rust_env: env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()),
        }
    }
}
