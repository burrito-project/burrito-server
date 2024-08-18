use sqlx::{Error as SqlxError, Pool, Postgres};
use std::thread::available_parallelism;

pub async fn create_pool() -> Result<Pool<Postgres>, SqlxError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(available_parallelism().unwrap().get() as u32)
        .connect(crate::env::DATABASE_URL.as_str())
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    Ok(pool)
}
