use crate::handlers::product_handler;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(product_handler::create_product))
            .route("", web::get().to(product_handler::get_all_products))
            .route("/{id}", web::get().to(product_handler::get_product_by_id))
            .route("/{id}", web::patch().to(product_handler::update_product))
            .route("/{id}", web::delete().to(product_handler::delete_product)),
    );
}
