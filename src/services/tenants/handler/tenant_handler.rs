use crate::services::tenants::dto::tenant_dto::TenantDto;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use crate::app_error::AppError;

#[axum::debug_handler]
pub async fn create_tenant(
    State(state): State<AppState>,
    Json(tenant): Json<TenantDto>,
) -> Result<(StatusCode, Json<TenantDto>), (StatusCode, AppError)> {
    let tenant = state
        .tenant_service
        .create_tenant(tenant)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok((StatusCode::CREATED, Json(tenant.into())))
}