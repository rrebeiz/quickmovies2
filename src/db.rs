use crate::config::Config;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(config: &Config) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(config.dsn.as_str())
        .await
        .expect("Failed to connect database pool")
}
