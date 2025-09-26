use sqlx::PgPool;
use std::sync::OnceLock;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn init() -> Result<(), sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/jh_database".to_string());
    let pool = PgPool::connect(&database_url).await?;
    DB_POOL.set(pool).unwrap();
    Ok(())
}

pub fn get_pool() -> &'static PgPool {
    DB_POOL.get().expect("Database not initialized")
}
