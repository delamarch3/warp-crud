use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;

pub type DB = Arc<PgPool>;

pub async fn db(max_connections: u32, uri: &str) -> DB {
    let pg_pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(uri)
        .await
        .unwrap();
    Arc::new(pg_pool)
}
