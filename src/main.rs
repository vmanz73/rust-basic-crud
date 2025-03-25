mod config;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use actix_web::{App, HttpServer, web};
use config::database::create_pool;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = create_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:3003")?
    .run()
    .await
}
