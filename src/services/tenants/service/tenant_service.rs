use std::collections::HashMap;
use std::iter::Filter;
use uuid::Uuid;
use crate::app_error::AppError;
use crate::services::tenants::dto::tenant_dto::{FetchRequest, TenantDto};
use crate::services::tenants::persistence::repository::TenantRepository;

#[derive(Clone)]
pub struct TenantService {
    tenant_repository: TenantRepository,
}

impl TenantService {
    pub fn new(tenant_repository: TenantRepository) -> TenantService {
        TenantService { tenant_repository }
    }
    pub async fn create_tenant(&self, tenant: TenantDto) -> Result<TenantDto, AppError> {
       
        let result = self.tenant_repository.insert(tenant).await?;
        let returned = TenantDto::from(result);
        Ok(returned)
    }

    pub async fn fetch_tenants(&self, filter: FetchRequest<Uuid>) -> Result<Vec<TenantDto>, AppError> {
                 self.tenant_repository.select(filter).await
    }


}
