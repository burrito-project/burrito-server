use sqlx::{Pool, Postgres};

pub async fn get_flag(pool: &Pool<Postgres>, name: &str, or_else: bool) -> bool {
    return sqlx::query_scalar!("SELECT value FROM flags WHERE name = $1 LIMIT 1", name)
        .fetch_one(pool)
        .await
        .unwrap_or(or_else);
}
