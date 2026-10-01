use crate::state::AppState;
use axum::{Router, routing::get};

pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Service startup successful" }))
        // later: .nest("/tenants", tenants::router())
        //        .nest("/products", products::router())
        .with_state(state)
}
