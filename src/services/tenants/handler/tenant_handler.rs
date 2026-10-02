use crate::app_error::AppError;
use crate::extractors::ValidatedJson;
use crate::services::tenants::dto::tenant_dto::TenantDto;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

#[axum::debug_handler]
pub async fn create_tenant(
    State(state): State<AppState>,
    ValidatedJson(tenant): ValidatedJson<TenantDto>,
) -> Result<(StatusCode, Json<TenantDto>), AppError> {
    let tenant = state.tenant_service.create_tenant(tenant).await?;

    Ok((StatusCode::CREATED, Json(tenant.into())))
}
