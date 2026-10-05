use crate::services::tenants::persistence::model::{TaxMode, Tenant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::{Validate, ValidationError};
#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct TenantDto {
    #[validate(length(
        min = "10",
        max = "100",
        message = "Please choose a valid business name"
    ))]
    pub name: String,
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,
    #[validate(custom(function = "validate_kra_pin"))]
    pub kra_pin: Option<String>,

    pub tax_mode: TaxMode,
    #[validate(length(equal = 3, message = "currency must be a 3-letter code"))]
    pub currency: String,
    #[validate(custom(function = "validate_timezone"))]
    pub timezone: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct FetchRequest<T> {
    pub page: usize,
    pub size: usize,
    pub id: Option<T>,
    pub criteria: Option<HashMap<String, String>>,
}

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let digits = phone
        .strip_prefix("+254")
        .ok_or_else(|| ValidationError::new("phone_format"))?;
    if digits.len() == 9 && digits.chars().all(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        Err(ValidationError::new("phone_format")
            .with_message("Invalid phone number format".into())
            .into())
    }
}
fn validate_kra_pin(pin: &str) -> Result<(), ValidationError> {
    let b = pin.as_bytes();
    let ok = b.len() == 11
        && b[0].is_ascii_alphabetic()
        && b[1..10].iter().all(u8::is_ascii_digit)
        && b[10].is_ascii_alphabetic();
    ok.then_some(()).ok_or_else(|| {
        ValidationError::new("kra_pin_format").with_message("Invalid KRA pin format".into())
    })
}

fn validate_timezone(tz: &str) -> Result<(), ValidationError> {
    tz.parse::<chrono_tz::Tz>().map(|_| ()).map_err(|_| {
        ValidationError::new("invalid_timezone").with_message("Invalid time zone format".into())
    })
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
