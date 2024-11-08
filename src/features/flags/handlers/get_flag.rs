use rocket::State;

use crate::core::AppState;
use crate::features::flags;

pub async fn get_flag_handler(flag: &str, state: &State<AppState>) -> Option<flags::schemas::Flag> {
    let flag = sqlx::query_as!(
        flags::schemas::Flag,
        "SELECT * FROM flags WHERE name = $1 AND internal = false LIMIT 1;",
        flag,
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    flag
}
