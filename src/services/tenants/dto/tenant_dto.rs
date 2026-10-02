use crate::services::tenants::persistence::model::{TaxMode, Tenant};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TenantDto {
    pub name: String,
    pub phone: String,
    pub kra_pin: Option<String>,
    pub tax_mode: TaxMode,
    pub currency: String,
    pub timezone: String,
}

impl From<Tenant> for TenantDto {
    fn from(t: Tenant) -> Self {
        Self {
            name: t.name,
            phone: t.phone,
            kra_pin: t.kra_pin,
            tax_mode: t.tax_mode,
            currency: t.currency,
            timezone: t.timezone,
        }
    }
}
