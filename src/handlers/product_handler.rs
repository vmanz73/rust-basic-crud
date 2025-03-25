use crate::models::product::ProductDTO;
use crate::services::product_service;
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

pub async fn create_product(
    pool: web::Data<PgPool>,
    product: web::Json<ProductDTO>,
) -> impl Responder {
    match product_service::create_product(&pool, product.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_all_products(pool: web::Data<PgPool>) -> impl Responder {
    match product_service::get_all_products(&pool).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_product_by_id(pool: web::Data<PgPool>, id: web::Path<String>) -> impl Responder {
    match product_service::get_product_by_id(&pool, id.into_inner()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_product(
    pool: web::Data<PgPool>,
    id: web::Path<String>,
    product: web::Json<ProductDTO>,
) -> impl Responder {
    match product_service::update_product(&pool, id.into_inner(), product.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_product(pool: web::Data<PgPool>, id: web::Path<String>) -> impl Responder {
    match product_service::delete_product(&pool, id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
