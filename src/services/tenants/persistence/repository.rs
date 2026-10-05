use crate::app_error::AppError;
use crate::services::tenants::dto::tenant_dto::{FetchRequest, TenantDto};
use crate::services::tenants::persistence::model::{Status, TaxMode, Tenant};
use sqlx::{AssertSqlSafe, PgPool};
use std::collections::HashMap;
use std::iter::Filter;
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
    pub async fn select(&self, filter: FetchRequest<Uuid>) -> Result<Vec<TenantDto>, AppError> {
        let mut selector = String::from("SELECT * FROM tenant.tenants");
        if filter.criteria.is_some() && !filter.criteria.clone().unwrap().is_empty() {
            selector.push_str(" WHERE ");
            for (i, (k, value)) in filter.criteria.clone().unwrap().into_iter().enumerate() {
                if i > 0 {
                    selector.push_str(" AND ");
                }
                selector.push_str(&format!(" {k} = ${}", i + 1));
            }
        }

        selector.push_str(" LIMIT ");
        selector.push_str(&filter.size.to_string());
        selector.push_str(" OFFSET ");
        selector.push_str((filter.size * filter.page).to_string().as_str());
        println!("Generated query {}", &selector);

        let mut q = sqlx::query_as::<_, Tenant>(AssertSqlSafe(selector));

        for (_, v) in filter.criteria.unwrap() {
            q = q.bind(v);
        }

        let result = q.fetch_all(&self.pg_pool).await.map_err(|t| {
            println!("Failed to fetch tenants from database {:?}", t);
            AppError::Conflict("Unable to select tenants.".to_string())
        })?;

        Ok(result.into_iter().map(TenantDto::from).collect())
    }
}
