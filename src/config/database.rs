use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env;

pub async fn create_pool() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set"),
        env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
        env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"),
        env::var("DATABASE_PORT").expect("DATABASE_PORT must be set"),
        env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
    );

    Pool::connect(&database_url)
        .await
        .expect("Failed to create pool")
}
