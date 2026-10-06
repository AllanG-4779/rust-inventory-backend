mod app;

mod app_error;
mod services;
mod state;
mod extractors;
mod dto;
mod db_commons;

use state::AppState;

#[tokio::main]
async fn main() {
    let envs = EnvironmentProperties::new();
    let pool = create_pool(&envs).await;
    if pool.is_err() {
        panic!("Failed to create pool");
    }
    let state = AppState::new(pool.unwrap(), envs);
    sqlx::migrate!("./migrations")
        .run(&state.pg_pool)
        .await
        .unwrap_or_else(|e| panic!("DB migration failed: {e}"));
    println!("DB migrated successfully!");

    let port = state.env.service_port;
    let app = app::build(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap_or_else(|e| panic!("Failed to bind port {port}: {e}"));
    println!("Server started on port {port}");

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| panic!("Server error: {e}"));
}

// db.rs
use crate::app_error::AppError;
use crate::state::EnvironmentProperties;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool(env: &EnvironmentProperties) -> Result<PgPool, AppError> {
    let db_url = format!(
        "{}://{}:{}@{}:{}/{}",
        env.db_driver, env.db_username, env.db_password, env.db_host, env.db_port, env.db_name
    );

    let pool = PgPoolOptions::new()
        .max_connections(env.db_pool_size as u32)
        .connect(&db_url)
        .await?;

    Ok(pool)
}
