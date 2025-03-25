mod config;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use actix_web::{App, HttpServer, web};
use config::database::create_pool;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = create_pool().await;
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init_routes)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
