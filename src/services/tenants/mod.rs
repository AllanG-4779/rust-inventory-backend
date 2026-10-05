use crate::state::AppState;
use axum::Router;
use axum::routing::{Route, post};

mod dto;
mod handler;
pub(crate) mod persistence;
pub mod service;

pub fn tenant_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handler::tenant_handler::create_tenant))
        .route("/fetch", post(handler::tenant_handler::fetch_tenants))
}
