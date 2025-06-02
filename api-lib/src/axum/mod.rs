use crate::{Env, init_db};
use axum::{Router, serve};
use log::{debug, error, info};
use sqlx::{Pool, Postgres};
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn axum_init<F, Fut>(router_fn: F)
where
    F: FnOnce(Pool<Postgres>) -> Fut,
    Fut: Future<Output = Router>,
{
    let env = Env::new();
    info!("Environment loaded with port: {}", env.port);

    let init_db = init_db().await.unwrap();
    info!("Database connection established");

    let router = router_fn(init_db).await;
    debug!("Router created successfully");

    let port = env.port;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on http://{}", addr);

    match serve(listener, router).await {
        Ok(_) => info!("Server stopped gracefully."),
        Err(err) => error!("Server encountered an error: {}", err),
    }
}
