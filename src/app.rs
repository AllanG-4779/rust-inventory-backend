use crate::services::tenants::tenant_routes;
use crate::state::AppState;
use axum::{Router, handler, routing::get};

pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/api/v1", get(|| async { "Service startup successful" }))
        .nest("/api/v1/tenants", tenant_routes())
        .with_state(state)
}
