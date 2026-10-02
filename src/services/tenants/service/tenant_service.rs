use crate::app_error::AppError;
use crate::services::tenants::dto::tenant_dto::TenantDto;
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
}
