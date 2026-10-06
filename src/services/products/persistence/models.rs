use crate::services::products::product_dto::ProductDto;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum MeasureType {
    Count,
    Weight,
    Volume,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Option<Uuid>,
    pub tenant_id: Uuid,
    pub name: String,
    pub sku: Option<String>,
    pub plu_code: Option<String>,
    pub measure_type: Option<MeasureType>,
    pub base_unit: Option<String>,
    pub item_cls_cd: Option<String>,
    pub tax_type_cd: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ProductDto> for Product {
    fn from(d: ProductDto) -> Self {
        Self {
            id: None,
            tenant_id: d.tenant_id,
            name: d.name,
            sku: None,
            plu_code: d.plu_code,
            measure_type: d.measure_type,
            base_unit: d.base_unit,
            item_cls_cd: None,
            tax_type_cd: "B".to_string(),
            active: false,
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

impl From<Product> for ProductDto {
    fn from(d: Product) -> Self {
        Self {
            name: d.name,
            sku: d.sku,
            tenant_id: d.tenant_id,
            base_unit: d.base_unit,
            plu_code: d.plu_code,
            item_cls_cd: d.item_cls_cd,
            measure_type: d.measure_type,
            tax_type_cd: Option::from(d.tax_type_cd),
        }
    }
}
