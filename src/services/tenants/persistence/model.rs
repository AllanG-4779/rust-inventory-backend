use crate::app_error::AppError;
use crate::services::tenants::dto::tenant_dto::TenantDto;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, sqlx::Type, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[sqlx(rename_all = "UPPERCASE", type_name = "text")]
pub enum TaxMode {
    #[sqlx(rename = "NONE")]
    Na,
    Vat,
    Tot,
}

#[derive(Debug, PartialEq, Eq, sqlx::Type)]
#[sqlx(rename_all = "UPPERCASE", type_name = "text")]
pub(crate) enum Status {
    Active,
    Suspended,
}
#[derive(sqlx::FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub kra_pin: Option<String>,
    pub tax_mode: TaxMode,
    pub currency: String,
    pub timezone: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tenant {
    pub fn new(tenant: TenantDto) -> Result<Self, AppError> {
        Self::parse_tz(&tenant.timezone)?;

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            name: tenant.name,
            phone: tenant.phone,
            kra_pin: tenant.kra_pin,
            tax_mode: tenant.tax_mode,
            currency: tenant.currency,
            timezone: tenant.timezone,
            status: Status::Active,
            created_at: now,
            updated_at: now,
        })
    }

    fn parse_tz(s: &str) -> Result<Tz, AppError> {
        s.parse::<Tz>()
            .map_err(|_| AppError::BadRequest("Invalid time zone".to_string()))
    }
}
