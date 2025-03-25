use crate::models::product::{Product, ProductDTO};
use sqlx::{PgPool, Row};

pub async fn create_product(pool: &PgPool, product: ProductDTO) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO product (name, stock, description) VALUES ($1, $2, $3)",
        product.name,
        product.stock,
        product.description
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    let rows = sqlx::query_as!(Product, "SELECT id, name, stock, description FROM product")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn get_product_by_id(pool: &PgPool, id: String) -> Result<Option<Product>, sqlx::Error> {
    let row = sqlx::query_as!(
        Product,
        "SELECT id, name, stock, description FROM product WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn update_product(
    pool: &PgPool,
    id: String,
    product: ProductDTO,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE product SET name = $1, stock = $2, description = $3 WHERE id = $4",
        product.name,
        product.stock,
        product.description,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_product(pool: &PgPool, id: String) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM product WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}
