use crate::app_error::AppError;
use crate::services::products::product_dto::ProductDto;
use sqlx::PgPool;

pub struct ProductRepository {
    pub pg_pool: PgPool,
}

impl ProductRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn create_product(&self, product: ProductDto) -> Result<ProductDto, AppError> {
        sqlx::query!(
            r#"INSERT INTO product.products (tenant_id, name, base_unit ) VALUES ($1, $2, $3)"#,
            product.tenant_id,
            product.name,
            product.base_unit
        )
        .execute(&self.pg_pool)
        .await?;

        Ok(product)
    }
}
