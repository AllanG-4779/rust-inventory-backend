use crate::services::products::persistence::repository::ProductRepository;

pub struct ProductService {
    product_repository: ProductRepository,
}

impl ProductService {
    pub fn new(product_repository: ProductRepository) -> ProductService {
        Self { product_repository }
    }

    pub fn create_user ()
}
