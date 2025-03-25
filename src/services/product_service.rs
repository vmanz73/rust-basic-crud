use crate::models::product::{Product, ProductDTO};
use crate::repositories::product_repository;
use sqlx::PgPool;

pub async fn create_product(pool: &PgPool, product: ProductDTO) -> Result<(), sqlx::Error> {
    product_repository::create_product(pool, product).await
}

pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    product_repository::get_all_products(pool).await
}

pub async fn get_product_by_id(pool: &PgPool, id: String) -> Result<Option<Product>, sqlx::Error> {
    product_repository::get_product_by_id(pool, id).await
}

pub async fn update_product(
    pool: &PgPool,
    id: String,
    product: ProductDTO,
) -> Result<(), sqlx::Error> {
    product_repository::update_product(pool, id, product).await
}

pub async fn delete_product(pool: &PgPool, id: String) -> Result<(), sqlx::Error> {
    product_repository::delete_product(pool, id).await
}
