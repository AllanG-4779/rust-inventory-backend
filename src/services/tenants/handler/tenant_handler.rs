use crate::app_error::AppError;
use crate::dto::common_wrappers::UniversalResponse;
use crate::extractors::ValidatedJson;
use crate::services::tenants::dto::tenant_dto::{FetchRequest, TenantDto};
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create_tenant(
    State(state): State<AppState>,
    ValidatedJson(tenant): ValidatedJson<TenantDto>,
) -> Result<(StatusCode, Json<UniversalResponse<TenantDto>>), AppError> {
    let tenant = state.tenant_service.create_tenant(tenant).await?;

    Ok((
        StatusCode::CREATED,
        Json(UniversalResponse::success(tenant)),
    ))
}

pub async fn fetch_tenants(
    State(state): State<AppState>,
    ValidatedJson(fetch): ValidatedJson<FetchRequest<Uuid>>,
) -> Result<Json<UniversalResponse<Vec<TenantDto>>>, AppError> {
    let result = state.tenant_service.fetch_tenants(fetch).await?;
    Ok(Json(UniversalResponse::success(result)))
}
