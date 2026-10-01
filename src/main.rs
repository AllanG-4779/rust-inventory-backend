mod app;
mod app_state;
mod db;

use app_state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new().await;

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
