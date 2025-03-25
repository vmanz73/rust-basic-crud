use actix_web::web;

mod product_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(product_routes::init));
}
