use crate::app_error::AppError;
use crate::services::tenants::dto::tenant_dto::TenantDto;
use crate::services::tenants::persistence::model::{Status, TaxMode, Tenant};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct TenantRepository {
    pub pg_pool: PgPool,
}

impl TenantRepository {
    pub fn new(pg_pool: PgPool) -> TenantRepository {
        TenantRepository { pg_pool }
    }

    pub async fn insert(&self, tenant: TenantDto) -> Result<Tenant, AppError> {
        let t = Tenant::new(tenant)?;
        sqlx::query!(
            r#"INSERT INTO tenant.tenants (id, name, phone, kra_pin, tax_mode, currency, timezone, status, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            t.id,
            t.name,
            t.phone,
            t.kra_pin,
            &t.tax_mode as &TaxMode,
            t.currency,
            t.timezone,
            &t.status as &Status,
            t.created_at,
            t.updated_at,
        )
            .execute(&self.pg_pool)
            .await
            .map_err(|e| e)?;

        Ok(t)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Tenant>, AppError> {
        sqlx::query_as!(
            Tenant,
            r#"SELECT id, name, phone, kra_pin,
                      tax_mode as "tax_mode: TaxMode", currency, timezone,
                      status as "status: Status", created_at, updated_at
               FROM tenant.tenants WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pg_pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn find_by_kra(&self, kra: &str) -> Result<Option<Tenant>, AppError> {
        sqlx::query_as!(
            Tenant,
            r#"SELECT id, name, phone, kra_pin,
                      tax_mode as "tax_mode: TaxMode", currency, timezone,
                      status as "status: Status", created_at, updated_at
               FROM tenant.tenants WHERE kra_pin = $1"#,
            kra
        )
        .fetch_optional(&self.pg_pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn find_by_phone(&self, phone: &str) -> Result<Option<Tenant>, AppError> {
        sqlx::query_as!(
            Tenant,
            r#"SELECT id, name, phone, kra_pin,
                      tax_mode as "tax_mode: TaxMode", currency, timezone,
                      status as "status: Status", created_at, updated_at
               FROM tenant.tenants WHERE phone = $1"#,
            phone
        )
        .fetch_optional(&self.pg_pool)
        .await
        .map_err(|e| e.into())
    }
}
