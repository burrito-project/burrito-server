use sqlx::{Pool, Postgres};

async fn create_flag_if_not_exists(
    pool: &Pool<Postgres>,
    name: &str,
    display_name: &str,
    value: bool,
    protected: bool,
    internal: bool,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
        INSERT INTO flags (name, display_name, value, protected, internal)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (name)
        DO NOTHING;
        "#,
        name,
        display_name,
        value,
        protected,
        internal,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[allow(dead_code)]
const INTERNAL: bool = true;
const NOT_INTERNAL: bool = false;

#[allow(dead_code)]
const PROTECTED: bool = true;
const NOT_PROTECTED: bool = false;

pub async fn setup_base_flags(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    create_flag_if_not_exists(
        pool,
        "ads_random_order",
        "Anuncios en orden aleatorio",
        true,
        NOT_INTERNAL,
        NOT_PROTECTED,
    )
    .await?;

    Ok(())
}
