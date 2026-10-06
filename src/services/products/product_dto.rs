use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::products::persistence::models::MeasureType;


#[derive(Serialize, Debug, Deserialize)]
pub struct ProductDto {
    pub name: String,
    pub sku: Option<String>,
    pub tenant_id: Uuid,
    pub base_unit: Option<String>,
    pub plu_code: Option<String>,
    pub item_cls_cd: Option<String>,
    pub measure_type: Option<MeasureType>,
    pub tax_type_cd: Option<String>,
}
