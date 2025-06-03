use sqlx::{Pool, Postgres};

pub type AppState = Pool<Postgres>;
