use crate::state::AppState;
use axum::Router;
use axum::routing::post;

mod dto;
mod handler;
pub(crate) mod persistence;
pub mod service;

pub fn tenant_routes() -> Router<AppState> {
    Router::new().route("/create", post(handler::tenant_handler::create_tenant))
}
